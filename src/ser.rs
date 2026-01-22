/***************************************************************************
 *
 * cJSON FFI BINDING FOR RUST
 * Copyright (C) 2026 Antonio Salsi <passy.linux@zresa.it>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 ***************************************************************************/

use osal_rs_serde::Serializer;


 use crate::CJsonResult;
use crate::cjson::CJsonError;
 use crate::cjson::CJson;


pub struct JsonSerializer {
    obj: CJson,
}

impl Serializer for JsonSerializer {
    type Error =  CJsonError;
    
    fn serialize_bool(&mut self, v: bool) -> Result<(), Self::Error> {

        self.obj.add_bool_to_object("b", v).map_err(|a| CJsonError::InvalidOperation)?;

        Ok(())
    }
    
    fn serialize_u8(&mut self, v: u8) -> core::result::Result<(), Self::Error> {
        todo!()
    }
    
    fn serialize_i8(&mut self, v: i8) -> core::result::Result<(), Self::Error> {
        todo!()
    }
    
    fn serialize_u16(&mut self, v: u16) -> core::result::Result<(), Self::Error> {
        todo!()
    }
    
    fn serialize_i16(&mut self, v: i16) -> core::result::Result<(), Self::Error> {
        todo!()
    }
    
    fn serialize_u32(&mut self, v: u32) -> core::result::Result<(), Self::Error> {
        todo!()
    }
    
    fn serialize_i32(&mut self, v: i32) -> core::result::Result<(), Self::Error> {
        todo!()
    }
    
    fn serialize_u64(&mut self, v: u64) -> core::result::Result<(), Self::Error> {
        todo!()
    }
    
    fn serialize_i64(&mut self, v: i64) -> core::result::Result<(), Self::Error> {
        todo!()
    }
    
    fn serialize_u128(&mut self, v: u128) -> core::result::Result<(), Self::Error> {
        todo!()
    }
    
    fn serialize_i128(&mut self, v: i128) -> core::result::Result<(), Self::Error> {
        todo!()
    }
    
    fn serialize_f32(&mut self, v: f32) -> core::result::Result<(), Self::Error> {
        todo!()
    }
    
    fn serialize_f64(&mut self, v: f64) -> core::result::Result<(), Self::Error> {
        todo!()
    }
    
    fn serialize_bytes(&mut self, v: &[u8]) -> core::result::Result<(), Self::Error> {
        todo!()
    }
    

}


impl JsonSerializer {

    pub fn new_object() -> CJsonResult<Self> {
        Ok(JsonSerializer {
            obj: CJson::create_object()?,
        })
    }
}