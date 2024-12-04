use std::{fs::File, io::{Write,BufRead,BufReader}};
use clap::Parser;

#[derive(Parser)]
#[command(author="Rolyando Lio Suhendro", version="1.0", about="To do list app")]
struct Args {
    #[arg(short ='n', long="new")]
    new: bool,
    #[arg(short ='f', long="full")]
    full: bool,
}

fn main() {
    // Inisiasi
    print!("{esc}c", esc = 27 as char);
    let mut todolist:Vec<String> = vec![];
    let args = Args::parse();
    let mut file = File::options()
                            .read(true)
                            .write(true)
                            .open("data.txt")
                            .unwrap_or_else(|_err| File::create("data.txt")
                            .expect("Gagal load dan membuat file 'data.txt'"));
    membaca_data(&file, &mut todolist);

    // Start program
    if args.new {
        println!("Anda akan membuat ToDo list");
        membuat_todo(&mut file, &mut todolist);
        tampilkan_todo(&todolist);
        std::process::exit(0);
    }
    if args.full {
        loop {
            let opsi = menu_utama();
            if opsi == 1 {
                tampilkan_todo(&todolist);
            } else if opsi == 2 {
                membuat_todo(&mut file, &mut todolist);
                // println!("{:?}",&todolist);
                continue;
            } else {
                break;
            }
        }
        std::process::exit(0);
    }
    tampilkan_todo(&todolist);
}


fn menu_utama() -> i8 {
    println!("\nSelamat datang di program to do list");
    println!("Apa yang ingin anda lakukan?");
    loop {
        let mut answer = String::new();
        println!("1. Tampilkan To Do List\n2. Buat To Do List\n3. Exit");
        std::io::stdin().read_line(&mut answer).unwrap();
        answer.truncate(1);
        if answer == "1".to_string() {
            return 1;
        } else if answer == "2".to_string() {
            return 2;
        } else if answer == "3".to_string() {
            return 3;
        } else {
            println!("Berikan input yang benar!");
            continue;
        }
    }

}

fn tampilkan_todo(todolist: &Vec<String>) {
    if !todolist.is_empty() {
        let mut count: i8 = 1;
        for todo in todolist {
            println!("{}.  {}",count,todo);
            count += 1;
        }
    } else {
        println!("To do anda tidak ada! pertimbangkan untuk membuat to do!\n");
    }
}

fn membuat_vec_todo(todolist: &mut Vec<String>){
    loop {
        let mut todo = String::new();
        let mut wanna_start: String = String::new();
        println!("Masukan To do anda");
        std::io::stdin().read_line(&mut todo).unwrap();
        todo.truncate(todo.len()-1);
        todolist.push(todo.clone());
        println!("Apakah anda ingin menambah todo?(Y/n)");
        std::io::stdin().read_line(&mut wanna_start).unwrap();
        wanna_start.truncate(1);
        if wanna_start == "y" {
            continue;
        } else if wanna_start == "n" {
            break;
        } else {
            println!("Kita tidak tahu apa yang kamu ketikan");
            break;
        }
    }
}


fn menulis_data(file: &mut File, todolist: &mut Vec<String>) {
    println!("Anda akan membuat data.txt");
    for todo in todolist {
        todo.push_str("\n");
        let len = file.write(todo.as_bytes()).expect("write fail");
        println!("Berhasil membuat sejumlah : {len} bytes");
    }
}

fn membaca_data(file: &File, todolist: &mut Vec<String>) {
    println!("Anda akan load data.txt");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        
        match line {
            Result::Ok(value) => todolist.push(value),
            Result::Err(_) => break
        }
    }
    println!("Load data.txt berhasil");
}

fn membuat_todo(file: &mut File, todolist: &mut Vec<String>) {
    membuat_vec_todo(todolist);
    menulis_data(file, todolist);
    println!("To do berhasil dibuat!");
}