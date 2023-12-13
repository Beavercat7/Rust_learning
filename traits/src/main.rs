fn main() {
    //一旦实现了trait,我们就可以用与NewsArticle和Tweet实例的非trait方法一样的方式调用trait方法了
    let tweet = Tweet{
     username:String::from("horse_ebooks");
     content:String::from("of course, as you probably already know, people"),
     reply:false,
     retweet:false,
    };
    println!("1 new tweet: {}",tweet.summarize());
 }