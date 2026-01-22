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

use osal_rs::utils::bytes_to_hex_into_slice;
use osal_rs_serde::Serializer;


use crate::CJsonResult;
use crate::cjson::CJsonError;
use crate::cjson::CJson;
use alloc::string::String;


pub struct JsonSerializer {
    obj: CJson,
}


impl Serializer for JsonSerializer {
    type Error =  CJsonError;

    fn serialize_bool(&mut self, name: &str, v: bool) -> core::result::Result<(), Self::Error> {
        self.obj.add_bool_to_object(name, v)?;

        Ok(())
    }
    
    fn serialize_u8(&mut self, name: &str, v: u8) -> core::result::Result<(), Self::Error> {
        self.obj.add_number_to_object(name, v as f64)?;

        Ok(())
    }
    
    fn serialize_i8(&mut self, name: &str, v: i8) -> core::result::Result<(), Self::Error> {
        self.obj.add_number_to_object(name, v as f64)?;

        Ok(())
    }
    
    fn serialize_u16(&mut self, name: &str, v: u16) -> core::result::Result<(), Self::Error> {
        self.obj.add_number_to_object(name, v as f64)?;

        Ok(())
    }
    
    fn serialize_i16(&mut self, name: &str, v: i16) -> core::result::Result<(), Self::Error> {
        self.obj.add_number_to_object(name, v as f64)?;

        Ok(())
    }
    
    fn serialize_u32(&mut self, name: &str, v: u32) -> core::result::Result<(), Self::Error> {
        self.obj.add_number_to_object(name, v as f64)?;

        Ok(())
    }
    
    fn serialize_i32(&mut self, name: &str, v: i32) -> core::result::Result<(), Self::Error> {
        self.obj.add_number_to_object(name, v as f64)?;

        Ok(())
    }
    
    fn serialize_u64(&mut self, name: &str, v: u64) -> core::result::Result<(), Self::Error> {
        self.obj.add_number_to_object(name, v as f64)?;

        Ok(())
    }
    
    fn serialize_i64(&mut self, name: &str, v: i64) -> core::result::Result<(), Self::Error> {
        self.obj.add_number_to_object(name, v as f64)?;

        Ok(())
    }
    
    fn serialize_u128(&mut self, name: &str, v: u128) -> core::result::Result<(), Self::Error> {
        self.obj.add_number_to_object(name, v as f64)?;

        Ok(())
    }
    
    fn serialize_i128(&mut self, name: &str, v: i128) -> core::result::Result<(), Self::Error> {
        self.obj.add_number_to_object(name, v as f64)?;

        Ok(())
    }
    
    fn serialize_f32(&mut self, name: &str, v: f32) -> core::result::Result<(), Self::Error> {
        self.obj.add_number_to_object(name, v as f64)?;

        Ok(())
    }
    
    fn serialize_f64(&mut self, name: &str, v: f64) -> core::result::Result<(), Self::Error> {
        self.obj.add_number_to_object(name, v)?;

        Ok(())
    }
    
    fn serialize_bytes(&mut self, name: &str, v: &[u8]) -> core::result::Result<(), Self::Error> {
        
        let mut buffer = String::with_capacity(v.len() * 2);

        unsafe {
            bytes_to_hex_into_slice(v, buffer.as_bytes_mut());
        }
        
        self.obj.add_string_to_object(name, &buffer)?;

        Ok(())
    }
    
    

}


impl JsonSerializer {

    pub fn new_object() -> CJsonResult<Self> {
        Ok(JsonSerializer {
            obj: CJson::create_object()?,
        })
    }
}