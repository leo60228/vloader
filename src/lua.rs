use detour::RawDetour;
use libffi::low::ffi_cif;
use libffi::middle::{Cif, Closure};
use libffi::raw::*;
use rlua::{Function, Lua, RegistryKey};
use std::collections::HashMap;
use std::convert::TryInto;
use std::mem::ManuallyDrop;
use std::os::raw::*;
use std::sync::Arc;

unsafe extern "C" fn lua_closure_callback(
    cif: &ffi_cif,
    result: &mut u8,
    args: *const *const c_void,
    data: &LuaClosureData,
) {
    let result = result as *mut u8 as *mut c_void;
    data.lua.context(|ctx| {
        let callback: Function = ctx.registry_value(&data.key).unwrap();
        let args_repr = std::slice::from_raw_parts(cif.arg_types, cif.nargs as usize);
        let args_vals = std::slice::from_raw_parts(args, cif.nargs as usize);
        let lua_args: Vec<_> = args_repr
            .into_iter()
            .zip(args_vals)
            .map(|(&repr, &val)| match (*repr).type_ as _ {
                FFI_TYPE_COMPLEX => unimplemented!("rust does not support complex"),
                FFI_TYPE_INT => rlua::Value::Integer((*(val as *const c_int)).try_into().unwrap()),
                FFI_TYPE_FLOAT => {
                    rlua::Value::Number((*(val as *const c_float)).try_into().unwrap())
                }
                FFI_TYPE_DOUBLE => rlua::Value::Number(*(val as *const c_double)),
                FFI_TYPE_LONGDOUBLE => unimplemented!("rust does not support long double"),
                FFI_TYPE_POINTER => {
                    rlua::Value::Integer((*(val as *const isize)).try_into().unwrap())
                }
                FFI_TYPE_SINT8 => rlua::Value::Integer((*(val as *const i8)).try_into().unwrap()),
                FFI_TYPE_SINT16 => rlua::Value::Integer((*(val as *const i16)).try_into().unwrap()),
                FFI_TYPE_SINT32 => rlua::Value::Integer((*(val as *const i32)).try_into().unwrap()),
                FFI_TYPE_SINT64 => rlua::Value::Integer(*(val as *const i64)),
                FFI_TYPE_STRUCT => unimplemented!(),
                FFI_TYPE_UINT8 => rlua::Value::Integer((*(val as *const u8)).try_into().unwrap()),
                FFI_TYPE_UINT16 => rlua::Value::Integer((*(val as *const u16)).try_into().unwrap()),
                FFI_TYPE_UINT32 => rlua::Value::Integer((*(val as *const u32)).try_into().unwrap()),
                FFI_TYPE_UINT64 => rlua::Value::Integer((*(val as *const u64)).try_into().unwrap()),
                FFI_TYPE_VOID => unreachable!(),
                _ => unreachable!(),
            })
            .collect();
        let res: rlua::Value = callback.call(lua_args).unwrap();
        #[rustfmt::skip]
        match (*cif.rtype).type_ as _ {
            FFI_TYPE_COMPLEX => unimplemented!("rust does not support complex"),
            FFI_TYPE_INT => *(result as *mut c_int) = ctx.coerce_integer(res).unwrap().unwrap().try_into().unwrap(),
            FFI_TYPE_FLOAT => *(result as *mut c_float) = ctx.coerce_number(res).unwrap().unwrap() as _,
            FFI_TYPE_DOUBLE => *(result as *mut c_double) = ctx.coerce_number(res).unwrap().unwrap(),
            FFI_TYPE_LONGDOUBLE => unimplemented!("rust does not support long double"),
            FFI_TYPE_POINTER => *(result as *mut isize) = ctx.coerce_integer(res).unwrap().unwrap().try_into().unwrap(),
            FFI_TYPE_SINT8 => *(result as *mut i8) = ctx.coerce_integer(res).unwrap().unwrap().try_into().unwrap(),
            FFI_TYPE_SINT16 => *(result as *mut i16) = ctx.coerce_integer(res).unwrap().unwrap().try_into().unwrap(),
            FFI_TYPE_SINT32 => *(result as *mut i32) = ctx.coerce_integer(res).unwrap().unwrap().try_into().unwrap(),
            FFI_TYPE_SINT64 => *(result as *mut i64) = ctx.coerce_integer(res).unwrap().unwrap(),
            FFI_TYPE_STRUCT => unimplemented!(),
            FFI_TYPE_UINT8 => *(result as *mut u8) = ctx.coerce_integer(res).unwrap().unwrap().try_into().unwrap(),
            FFI_TYPE_UINT16 => *(result as *mut u16) = ctx.coerce_integer(res).unwrap().unwrap().try_into().unwrap(),
            FFI_TYPE_UINT32 => *(result as *mut u32) = ctx.coerce_integer(res).unwrap().unwrap().try_into().unwrap(),
            FFI_TYPE_UINT64 => *(result as *mut u64) = ctx.coerce_integer(res).unwrap().unwrap().try_into().unwrap(),
            FFI_TYPE_VOID => {}
            _ => unreachable!(),
        }
    });
}

pub struct LuaClosureData {
    pub key: RegistryKey,
    pub lua: Arc<Lua>,
}

rental! {
    mod lua_closure {
        use super::LuaClosureData;
        use std::sync::Arc;
        use rlua::RegistryKey;
        use libffi::middle::Closure;

        #[rental]
        pub struct LuaClosure {
            data: Arc<LuaClosureData>,
            closure: Closure<'data>,
        }
    }
}

pub struct LuaClosure(lua_closure::LuaClosure);

impl LuaClosure {
    pub fn new(data: Arc<LuaClosureData>, cif: Cif) -> Self {
        Self(lua_closure::LuaClosure::new(data, |data_ref| {
            Closure::new(cif, lua_closure_callback, data_ref)
        }))
    }

    pub fn code_ptr(&self) -> &unsafe extern "C" fn() {
        self.0.ref_rent(|c| c.code_ptr())
    }

    pub unsafe fn instantiate_code_ptr<T>(&self) -> &T {
        self.0.ref_rent(|c| c.instantiate_code_ptr())
    }
}

pub struct LuaHook {
    pub detour: RawDetour,
    closure: ManuallyDrop<Box<LuaClosure>>,
}

impl LuaHook {
    pub fn new(detour: RawDetour, closure: LuaClosure) -> Self {
        Self {
            detour,
            closure: ManuallyDrop::new(Box::new(closure)),
        }
    }

    pub fn closure(&self) -> &LuaClosure {
        &**self.closure
    }

    pub fn closure_mut(&mut self) -> &mut LuaClosure {
        &mut **self.closure
    }
}

impl Drop for LuaHook {
    fn drop(&mut self) {
        unsafe {
            self.detour.disable().unwrap();
            ManuallyDrop::drop(&mut self.closure);
        }
    }
}

pub struct LuaState {
    pub lua: Lua,
    pub hooks: HashMap<String, LuaHook>,
}
