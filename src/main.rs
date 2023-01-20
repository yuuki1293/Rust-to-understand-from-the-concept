trait SayHello {
    fn say_hello(&self);
}

trait SayThankyou {
    fn say_thankyou(&self);
}

struct EnglishPerson;
struct SpanishPerson;

impl SayHello for EnglishPerson {
    fn say_hello(&self) {
        println!("Hello");
    }
}

impl SayThankyou for EnglishPerson {
    fn say_thankyou(&self) {
        println!("Thank you");
    }
}

impl SayHello for SpanishPerson {
    fn say_hello(&self) {
        println!("Hola");
    }
}

impl SayThankyou for SpanishPerson {
    fn say_thankyou(&self) {
        println!("Gracias");
    }
}

fn say_hello_general<T: SayHello>(speaker: &T) {
    speaker.say_hello();
}

fn say_thankyou_general<T: SayThankyou>(speaker: &T) {
    speaker.say_thankyou();
}

fn main() {
    let en = EnglishPerson;
    let sp = SpanishPerson;

    say_hello_general(&en);
    say_thankyou_general(&sp);

    say_thankyou_general(&en);
    say_thankyou_general(&sp);
}
