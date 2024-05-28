use tokio::join;
use  tokio::net::TcpListener;
use  tokio::io::{AsyncBufReadExt, AsyncWriteExt,};
use  std::io;

#[tokio::main]

async fn  main() {
    let   alphabe= ["A", "B", "C", "d", "E", "F", "g", "h", "i", "j", "k", "l ", "m", "n", " O", " P", "q", " R ", " S", " T", " U", " V", " W", " X", " Y", "  Z"];

    println!("entrez le message  a faire  chiffrer     ou  a faire  code   par mon  script  ");

    let  mut  nessage  = String::new();
    io::stdin().read_line(&mut  nessage).expect("erreur");
    let   nessage : String   =  nessage.trim().to_uppercase();
    let  mut  stoker : Vec<char> = vec![];
    
        stoker.push(nessage.chars().nth(0).unwrap());
        println!("{:?}", stoker);
    
    //  commencons  le  chiffremment  pas  subtittutions de caractere

      for i in   0..stoker.len() {
        for j in 0..alphabe.len() {

            if   stoker[i] == alphabe[j].chars().next().unwrap() {
                let  encrypter_index = (j + 3)%alphabe.len();
                stoker[j] = alphabe[encrypter_index].chars().next().unwrap();

            }
        }
      }

    println!("le code  apres le  chiffremeent  est  {:#?}",stoker.iter().collect::<String>() );

}