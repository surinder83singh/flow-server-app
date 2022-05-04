use flow_server::router::Router;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let cwd = std::env::current_dir().unwrap();
    let router = Router::new(cwd);

    const TIDE_SECRET:&[u8] = "5b1f39e4511df9ea5a224760bec569ae08e4e90b93c6eaf97ddc1d79753b4fa7".as_bytes();

    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        TIDE_SECRET
        /*std::env::var("TIDE_SECRET")
            .expect(
                "Please provide a TIDE_SECRET value of at \
                      least 32 bytes in order to run this example",
            )
            .as_bytes(),*/
    ));

    app.with(tide::utils::Before(
        |mut request: tide::Request<()>| async move {
            let session = request.session_mut();
            let visits: usize = session.get("visits").unwrap_or_default();
            session.insert("visits", visits + 1).unwrap();
            request
        },
    ));

    /*
    app.at("/").get(|req: tide::Request<()>| async move {
        let visits: usize = req.session().get("visits").unwrap();
        Ok(format!("you have visited this website {} times", visits))
    });
    */

    app.at("/").serve_file("www/index.html")?;

    app.at("/reset")
        .get(|mut req: tide::Request<()>| async move {
            req.session_mut().destroy();
            Ok(tide::Redirect::new("/"))
        });

    app.at("/").serve_dir("www/")?;

    router.init(&mut app);
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
