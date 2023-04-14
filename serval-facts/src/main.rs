use rand::{thread_rng, Rng};

fn main() {
    // Facts from https://marybatessciencewriter.com/home/2016/01/04/the-creature-feature-10-fun-facts-about-servals
    let facts = String::from("Servals are leggy. Servals have the longest legs of any cat relative to their body size. These extra-long legs, plus their elongated necks, give them the nickname of \"giraffe cat.\" Much of their legginess is due to stretched out metatarsal bones in the feet.
        They have the largest ears of any cat. Atop their small heads, servals have tall, oval ears, which are black on the back with a distinctive white spot. If humans had ears like servals, they would be as big as dinner plates.
        Servals can jump. When going after birds and insects, servals make high vertical leaps into the air — sometimes jumping more than 9 feet straight up. They can also leap up to 12 feet horizontally from a stationary position to land on targets with enough force to stun or kill their prey on impact.
        Servals communicate with pee. Both males and females mark their territories by spraying urine on trees and bushes, scraping fresh urine into the ground with their claws, and rubbing their cheek glands on the ground or brush. Males tend to mark their territory more frequently than females, spraying up to 46 times per hour or 41 times per square kilometer. One male was recorded marking 566 times in a four hour period, when he was following a female.
        Servals have cat fights. Threat displays between hostile servals can look scary, with the cats flattening their ears, arching their backs, baring their teeth, and nodding their heads vigorously. If the situation escalates, they lash out with their long front legs and bark and growl.
        Servals can purr. In addition to barks and growls, the serval can purr, chirp, hiss, cackle, grunt, and meow. Listen to some serval sounds at the 2:00 minute mark here.
        Servals can live more than 20 years. Life expectancy in the wild is only about 10 years, but servals can more than double that in captivity.
        Servals are stalk-and-pounce hunters. Servals rely heavily on their keen sense of hearing and height advantage to locate prey in tall grass. They pounce from one to four meters away, landing on the prey with their front paws and then delivering a deadly bite to the neck. Servals can also leap in the air to grab birds in flight, hook fish and frogs out of the water, and snatch rodents from out of their burrows. It's estimated that the cats are successful in about half of their prey capture attempts.
        Servals are solitary except for mating and mothers with kittens. Males and females come together for a few days when the female is in heat; she alerts males in the area with short and sharp calls or long yowls. After about 70 days, the kittens are born in a den or burrow. Once the males can hunt for themselves, at about six months old, the mother chases them off her territory. She tolerates the presence of female kittens for a little while longer.
        There is a serval-domestic cat hybrid. It's called the Savannah, and it's a cross between a serval and a regular domestic cat. The breed was recognized by the International Cat Association in 2001. Although kept as pets, they retain a lot of their wild, serval appearance and behaviors. Owning a Savannah cat is not legal in every state.");
    let facts: Vec<_> = facts.split('\n').collect();

    // Pick a fact
    let mut rng = thread_rng();
    let range = 0..(facts.len() - 1);
    let i = rng.gen_range(range);
    let fact = facts[i].trim();

    // Word wrap it so it's readable as plain text
    const WORD_WRAP_WIDTH: usize = 80;
    let mut wrapped_fact = String::from("");
    let mut line_length = 0;
    for word in fact.split_ascii_whitespace() {
        if line_length + word.len() >= WORD_WRAP_WIDTH {
            wrapped_fact.push_str("\n");
            line_length = 0;
        } else if !wrapped_fact.is_empty() {
            wrapped_fact.push_str(" ");
            line_length += 1;
        }

        wrapped_fact.push_str(word);
        line_length += word.len();
    }

    println!("HTTP/1.0 200 OK");
    println!("Content-Type: text/plain");
    println!();
    println!("{}", wrapped_fact);
}
