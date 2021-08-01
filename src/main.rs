use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use mysql::prelude::*;
use mysql::*; // 用来处理日期

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
struct Student {
    id: usize,
    name: String,
    age: usize,
    id_card: String,
    last_modified: NaiveDate,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = Opts::from_url("mysql://root:12345678@127.0.0.1:3306/mydb").unwrap();
    let pool = Pool::new(url).unwrap(); // 获取连接池
    let mut conn = pool.get_conn().unwrap(); // 获取链接
    conn.query_iter("select * from student")
        .unwrap()
        .for_each(|row| {
            let r: (i32, String, i32, String, NaiveDate) = from_row(row.unwrap());
            println!("{}, {},{},{}, {:?}", r.0, r.1, r.2, r.3, r.4);
        });
    let res = conn
        .query_map(
            "select * from student",
            |(id, name, age, id_card, last_modified)| Student {
                id: id,
                name: name,
                age: age,
                id_card: id_card,
                last_modified: last_modified,
            },
        )
        .expect("failed");
    for item in res {
        println!("{}, {}, {}, {}, {:?}",item.id,item.name,item.age,item.id_card,item.last_modified);
    }
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:65534")?
    .run()
    .await
}
