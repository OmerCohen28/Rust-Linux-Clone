
pub mod linux{
    use std::env;
    use std::fs;
    use std::fs::File;
    use std::path;
    use std::path::Path;
    use std::path::PathBuf;
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::str::FromStr;

    pub fn pwd() -> String{
        match env::current_dir(){
            Ok(dir) => {
                let tmp = dir.to_str();
                match tmp {
                    Some(p) => String::from(p),
                    None => String::from("Could not read current directory")
                }
            },
            Err(err) => {
                String::from(format!("Encountered an Eror: {:?}",err))
            },
        }
    } 

    pub fn ls(p:&path::Path){
        let all = match fs::read_dir(&p){
            Ok(x) => x,
            Err(err) => {
                println!("Eror: {:?}",err);
                panic!("Can not read directory");
            }
        };
        for thing in all{
            match thing{
                Ok(dir) => println!("{:?}",dir.path()),
                Err(err) => println!("Encountered eror: {:?}",err),
            }
        }
    }

    pub fn cat(p:&PathBuf) -> std::io::Result<(String)>{
        if !p.is_file(){
            println!("Entity is not a file")
        }
        let file = File::open(p)?;
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content)?;
        Ok(content)
    }
    
    pub fn rm(p:&PathBuf)->std::io::Result<()>{
        if !p.is_file(){
            println!("Entity is not a file")
        }
        fs::remove_file(p)?;
        Ok(())
    }

    pub fn touch(name:&str)-> std::io::Result<()>{
        let file = File::create(name)?;
        Ok(())
    }

    //get old path and new path
    // create file with same name at new path
    //copy the file over with fs::copy()
    pub fn mv(name:&str,old_p:&PathBuf,new_p:&mut PathBuf)->Result<(), std::io::Error>{
        new_p.push(Path::new(name));
        let file = File::create(&new_p);
        if !new_p.is_file(){
            return Err(std::io::Error::new(std::io::ErrorKind::Other,"Tried to move a non file entity"));
        }
        let mut old_new_p = PathBuf::new();
        old_new_p.push(old_p);
        fs::copy(old_new_p, new_p)?;
        Ok(())
    }

    pub fn take_input(p:&mut PathBuf){
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).unwrap();
        str = String::from(str.trim());
        if &str[..] == "stop"{
            std::process::exit(0);
        }
        if &str[0..1]=="c" && &str[1..2]=="d"{
            match &str[..]{
                "cd .." => {
                    p.pop();
                    ()
                },
                _ => {
                    let mut it = str.split(" ").into_iter();
                    it.next();
                    let next = it.next();
                    match next{
                        Some(x) => p.push(x),
                        None => println!("Could not change directory"),
                    };
                    
                    if !p.exists(){
                        println!("Entity Not Found");
                        p.pop();
                    }

                    ()

                }
            }
        }

        if &str[0..2] == "ls"{
            ls(&p);
        }

        if &str[0..3] == "cat"{
            let args : Vec<&str> = str.split(" ").collect();
            println!("{:?}",args);
            if args.len() > 2 {
                println!("too many arguments given");
                return ;
            }
            if args.len() == 1{
                println!("you need to specify the file to cat as well");
                return ;
            }
            p.push(Path::new(args[1]));
            let outcome = cat(p);
            p.pop();
            match outcome{
                Ok(content) => println!("{}",content),
                Err(err) => println!("Encountered Eror: {}",err),
            }
        }
        
        if &str[0..2] == "rm"{
            let args : Vec<&str> = str.split(" ").collect();
            println!("{:?}",args);
            if args.len() > 2 {
                println!("too many arguments given");
                return ;
            }
            if args.len() == 1{
                println!("you need to specify the file to cat as well");
                return ;
            }
            p.push(Path::new(args[1]));
            let outcome = rm(p);
            p.pop();
            match outcome{
                Ok(()) => (),
                Err(err) => println!("Encountered Eror: {}",err),
            }
        }

        if &str[0..5] == "touch"{
            let args : Vec<&str> = str.split(" ").collect();
            if args.len() > 2 {
                println!("too many arguments given");
                return ;
            }
            if args.len() == 1{
                println!("you need to specify the file to touch as well");
                return ;
            }
            let outcome = touch(args[1]);
            match outcome{
                Ok(()) => (),
                Err(err) => println!("Encountered Eror: {}",err),
            }
        }

        if &str[0..2] == "mv"{
            let args : Vec<&str> = str.split(" ").collect();
            if args.len() > 3 {
                println!("too many arguments given");
                return ;
            }
            if args.len() <3{
                println!("you need to specify the file and the new location");
                return ;
            }
            let mut new_path = match PathBuf::from_str(args[2]){
                Ok(p) => p,
                Err(err) => {
                    println!("Encoutered Eror: {}",err);
                    return ;
                }
            };
            new_path.push(args[1]);
            let outcome = mv(args[1],p,&mut new_path);
            match outcome{
                Ok(()) => (),
                Err(err) => println!("Encoutered Eror: {}",err),
            }
        }

    }

}