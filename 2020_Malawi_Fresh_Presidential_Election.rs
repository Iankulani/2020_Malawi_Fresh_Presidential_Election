use std::io;

fn main() {
    println!("=============================== MALAWI ELECTROL COMMISSION ===============================\n");
    
    // Get total number of published centers
    const PERCENT: f64 = 100.0;
    let input = String::new();

    // Get total number of registered voters
    let mut input = String::new();
    println!("Enter Total Registered Voters/Turnout:");
    io::stdin().read_line(&mut input).unwrap();
    let total_registered_votes: i32 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter Total Votes Cast/Total Votes:");
    io::stdin().read_line(&mut input).unwrap();
    let total_votes_cast: i32 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter Total Null_&_Void Votes/Invalid Votes:");
    io::stdin().read_line(&mut input).unwrap();
    let null_and_void: i32 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter Total Valid Votes:");
    io::stdin().read_line(&mut input).unwrap();
    let total_valid_votes: i32 = input.trim().parse().unwrap();
    
 // Get total valid votes for each candidate
    input.clear();
    println!("Enter Total Valid Votes for Dr Lazarus Chakwera: ");
    io::stdin().read_line(&mut input).unwrap();
    let total_valid_votes_for_lazarus_chakwera: i32 = input.trim().parse().unwrap();
    
    input.clear();
    println!("Enter Total Valid Votes for Peter Driver Sinos Kuwani: ");
    io::stdin().read_line(&mut input).unwrap();
    let total_valid_votes_for_peter_kuwani: i32 = input.trim().parse().unwrap();
    
    input.clear();
    println!("Enter Total Valid Votes for Professor Arthur Peter Mutharika: ");
    io::stdin().read_line(&mut input).unwrap();
    let total_valid_votes_for_arthur_mutharika: i32 = input.trim().parse().unwrap();
    
    // Determine the winner
    if total_valid_votes_for_lazarus_chakwera > total_valid_votes / 2 {
        println!("Congratulations Dr Lazarus Maccathy Chakwera, you're a winner of the 2020 election\n");
    } else if total_valid_votes_for_peter_kuwani > total_valid_votes / 2 {
        println!("Congratulations Peter Kuwani, you're a winner of the 2020 election\n");
    } else if total_valid_votes_for_arthur_mutharika > total_valid_votes / 2 {
        println!("Congratulations Professor Arthur Peter Mutharika, you're a winner of the 2020 election\n");
    } else {
        println!("No majority winner was found. RUNOFF may be required\n");
    }

    println!("____________________________________ ELECTION STATISTICS ____________________________________\n");

    // Calculate percentages
    let percent = 100.0;

    let percentage_total_votes_cast = (total_votes_cast as f64 / total_votes_cast as f64) * percent;
    println!("Total Votes Cast in percentage = {:.2}%", percentage_total_votes_cast);

    let percentage_total_valid_votes = (total_valid_votes as f64 / total_votes_cast as f64) * percent;
    println!("Total Valid Votes for all candidates in percentage = {:.2}%", percentage_total_valid_votes);

    let percentage_null_and_void = (null_and_void as f64 / total_valid_votes as f64) * percent;
    println!("Total Null_&_Void votes in percentage = {:.2}%", percentage_null_and_void);

    let percentage_registered_voters = (total_votes_cast as f64 / total_registered_votes as f64) * percent;
    println!("Total Registered voters/turnout in percentage = {:.2}%", percentage_registered_voters);

    let percentage_lazarus_chakwera = (total_valid_votes_for_lazarus_chakwera as f64 / total_valid_votes as f64) * percent;
    println!("Total Valid Votes for Dr Lazarus Chakwera in Percentage = {:.2}%", percentage_lazarus_chakwera);

    let percentage_peter_kuwani = (total_valid_votes_for_peter_kuwani as f64 / total_valid_votes as f64) * percent;
    println!("Total Valid Votes for Peter Kuwani in percentage = {:.2}%", percentage_peter_kuwani);

    let arthur_mutharika_percentage = (total_valid_votes_for_arthur_mutharika as f64 / total_valid_votes as f64) * PERCENT;
    println!("Total Valid Votes for Professor Mutharika in percentage = {:.2}%", arthur_mutharika_percentage);

    println!("==========================================================================================\n");
}
