use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io::{self/*, Read, Write*/};
use std::error::Error;


fn init_file() -> Result<Value, Box<dyn std::error::Error>>{
    let file = File::open("database.json")?;
    let reader = io::BufReader::new(file);
    let data: Value = serde_json::from_reader(reader)?;
    Ok(data)
}


fn save_file(data: &Value) -> Result<(), io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("database.json")?;

    let writer = io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data)?;

    Ok(())
}


pub fn read_tasks(id:String) -> String{
    let mut data = init_file().unwrap_or(Value::Null);
    let mut s = "Список задач:\n".to_string();

    if let Some(vlue) = data.get_mut(id) {
        if let Some(vctr) = vlue.as_array(){
            if vctr.is_empty(){
                return "Нет текущих задач".to_string();
            }
            for i in 0..(vctr.len()){
                s.push_str((i+1).to_string().as_str());
                s.push_str(". ");
                let t = vctr[i].to_string();
                s.push_str(&t[1..t.len()-1]);
                s.push_str("\n");
            }
        }
        else{
            return "Ошибка при чтении задач".to_string();
        }
    }
    else{
        return "Нет текущих задач".to_string();
    }

    return s.to_string();
    // println!("{:?}",data.get_mut(id).unwrap().as_array().unwrap());
    // data.get_mut(id).unwrap().as_array().unwrap()
}


pub fn add_task(id: String, task: String) -> Result<(), Box<dyn Error>>{
    let mut data = init_file().unwrap_or(Value::Null);

    if let Some(vlue) = data.get_mut(id.clone()) {
        if let Some(vctr) = vlue.as_array_mut(){
            vctr.push(serde_json::to_value(task)?);
        }
        else{
            data[id] = serde_json::to_value([task])?;
        }
    }
    else{
        data[id] = serde_json::to_value([task])?;
    }

    save_file(&data)?;
    Ok(())
}


pub fn clear(id: String) -> Result<(), Box<dyn Error>>{
    let mut data = init_file().unwrap_or(Value::Null);
    // data[id] = serde_json::to_value([Value::Null;0])?;

    if let Some(vlue) = data.get_mut(id.clone()) {
        if let Some(vctr) = vlue.as_array_mut(){
            vctr.clear();
        }
    }
    save_file(&data)?;
    Ok(())
}


pub fn mark(id: String, number: usize) -> Result<(), Box<dyn Error>>{
    let mut data = init_file().unwrap_or(Value::Null);

    if let Some(vlue) = data.get_mut(id.clone()) {
        if let Some(vctr) = vlue.as_array_mut(){
            if number-1 < vctr.len(){
                vctr.remove(number-1);
            }
            else{
                return Err("Несуществующая задача")?;
            }
        }
    }
    save_file(&data)?;
    Ok(())
}