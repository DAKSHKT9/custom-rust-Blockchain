use clap::{arg, Command};

use crate::blockchain::Blockchain;
use crate::errors::Result;

pub struct Cli{
    bc: Blockchain
}

impl Cli{
    pub fn new() -> Result<Cli>{
        Ok(Cli { bc: Blockchain::new()? })
    }


    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-Rust-daksh")
            .version("v0.1")
            .author("Daksh Katkar")
            .about("My first custom chain")
            .subcommand(Command::new("printchain").about("Prints All chain Blocks"))
            .subcommand(Command::new("addblock").about("Adds new Block to Chain").arg(arg!(<DATA>" 'The Blockchain Data'")))
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("addblock"){
            if let Some(c) = matches.get_one::<String>("DATA") {
                self.addblock(String::from(c))?;
            } else {
                println!("Not printing testing list");
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("addblock"){
            self.printchain();
        }

        Ok(())


    }

    fn addblock(&mut self, data: String) -> Result<()>{
        self.bc.add_block(data);
        Ok(())
    }

    fn printchain(&self) -> Result<()>{
        for item in self.bc.iter(){
            println!("block: {:?}", item );
        }
        Ok(())
    }

}


