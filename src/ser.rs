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

use osal_rs::print;
use osal_rs::utils::bytes_to_hex_into_slice;
use osal_rs_serde::Serializer;

use crate::CJsonResult;
use crate::cjson::CJsonError;
use crate::cjson::CJson;

use alloc::vec::Vec;
use alloc::string::String;
use alloc::vec;


pub struct JsonSerializer<'a> {
    obj: &'a mut CJson,
    stack: Vec<CJson>,
    index: usize,
}


impl<'a> Serializer for JsonSerializer<'a> {
    type Error =  CJsonError;

    fn serialize_struct_start(&mut self, name: &str, _len: usize) -> core::result::Result<(), Self::Error> {

        print!("serialize_struct_start:{}\r\n", name);


        

        if name == "" {
            self.stack.push(self.obj.clone());

            Ok(())
        } else {
            // match self.obj.get_object_item(name) {
            //     Ok(_) => {
            //         // Object already exists, do nothing
            //     },
            //     Err(_) => {
            //         let name_string: String = name.into();
            //         let new_obj = CJson::create_object()?;
            //         self.obj.add_item_to_object(name, new_obj.clone())?;
                    

            //         self.containers_stack.push((name_string, new_obj.clone()));
            //         self.obj = new_obj;
            //     }
            // }

             if let Err(_) = self.obj.get_object_item(name) {
                // self.obj = CJson::create_object()?;
                // self.obj.add_item_to_object(name, self.obj.clone())?;
                // self.containers_stack.push(self.obj.clone());
            }
            

            Ok(())
        }


    }

    // fn serialize_field<T: Serialize>(&mut self, name: &str, value: &T) -> core::result::Result<(), Self::Error> {
    //     value.serialize(name, self)
    // }

    fn serialize_struct_end(&mut self) -> core::result::Result<(), Self::Error> {
        

        self.obj = &mut parent_obj;

        Ok(())
    }

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
    
    fn serialize_string(&mut self, name: &str, v: &String) -> core::result::Result<(), Self::Error> {
        self.obj.add_string_to_object(name, v)?;

        Ok(())
    }
    
    fn serialize_str(&mut self, name: &str, v: &str) -> core::result::Result<(), Self::Error> {
        self.obj.add_string_to_object(name, v)?;

        Ok(())
    }
    
    fn serialize_vec<T: osal_rs_serde::Serialize>(&mut self, name: &str, v: &alloc::vec::Vec<T>) -> core::result::Result<(), Self::Error> {
        for item in v.iter() {
            item.serialize(name, self)?;
        }
        Ok(())
    }
    
    fn serialize_array<T: osal_rs_serde::Serialize>(&mut self, name: &str, v: &[T]) -> core::result::Result<(), Self::Error> {
        for item in v.iter() {
            item.serialize(name, self)?;
        }
        Ok(())
    }
    
    

}


impl<'a> JsonSerializer<'a> {

    pub fn new() -> CJsonResult<Self> {

        let obj = CJson::create_object()?;

        Ok(Self {
            obj: &mut obj,
            stack: vec![obj],
            index: 0,
        })
    }

    pub fn print(self) -> CJsonResult<String> {
        self.obj.print()
    }

    pub fn print_unformatted(self) -> CJsonResult<String> {
        self.obj.print_unformatted()
    }
} 