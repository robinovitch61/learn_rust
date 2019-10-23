// make crates available to program 
extern crate iron;
#[macro_use] extern crate mime;
extern crate router;
extern crate urlencoded;

// make features in specific crate modules available
use iron::prelude::*; // prelude modules are meant to have *
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving on http://localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap(); // unwrap causes panic if Err
}

fn get_form(_request: &mut Request) -> IronResult<Response> { // _ specifies unused variable, no warnings
    let mut response = Response::new(); // Response comes from iron

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8)); // use mime macro to set content media type
    response.set_mut(r#"
        <title>Greatest Common Denominator Calculator</title>
        <form action="/gcd" method="post">
          <input type="text" name="n"/>
          <input type="text" name="n"/>
          <button type="submit">Compute GCD!</button>
        </form>
        "#); // r#" ... "# indicates start and end of raw string block
    
    Ok(response) // no ; here indicates return value at end of fun
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    // form_data will either be a response indicating error 
    // OR the body of the request parsed as a URLencoded query string
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map 
    };

   let unparsed_numbers = match form_data.get("n") {
       None => {
           response.set_mut(status::BadRequest);
           response.set_mut(format!("form data has no 'n' parameter\n"));
           return Ok(response);
       }
       Some(nums) => nums
   };

   let mut numbers = Vec::new();
   for unparsed in unparsed_numbers {
       match u64::from_str(&unparsed) {
           Err(_) => {
               response.set_mut(
                   format!("Value for 'n' parameter not a number: {:?}\n",
                           unparsed));
               return Ok(response);
           }
           Ok(n) => {
               numbers.push(n);
           }
       }
   }

   let mut d = numbers[0];
   for m in &numbers[1..]{
       d = gcd(d, *m);
   }

   response.set_mut(status::Ok);
   response.set_mut(mime!(Text/Html; Charset=Utf8));
   response.set_mut(
       format!("The greatest common divisor of the numbers {:?} is <b>{}</b>\n",
               numbers, d));
   Ok(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
