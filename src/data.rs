use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
    id: 1,
    name: "Samsung 55\" 4K UHD Smart TV".to_string(),
    price: 699.99,
    description: "Experience stunning picture quality with this Samsung 55-inch 4K UHD Smart TV. Features HDR support and built-in streaming apps.".to_string(),
    image: "/tv.jpg".to_string()
},
Product {
    id: 2,
    name: "Sony WH-1000XM5 Noise Cancelling Headphones".to_string(),
    price: 399.99,
    description: "Industry-leading noise cancellation with premium sound quality. Wireless, comfortable, and perfect for travel or work.".to_string(),
    image: "/headphones.jpg".to_string()
},
Product {
    id: 3,
    name: "Apple MacBook Air 13\" (M2 Chip)".to_string(),
    price: 1099.99,
    description: "Ultra-thin and powerful laptop featuring Apple’s M2 chip, all-day battery life, and a brilliant Retina display.".to_string(),
    image: "/macbook.jpg".to_string()
},
Product {
    id: 4,
    name: "Dell XPS 15 Laptop".to_string(),
    price: 1499.99,
    description: "High-performance laptop with a 15-inch InfinityEdge display, ideal for creators and power users.".to_string(),
    image: "/laptop.jpg".to_string()
},
Product {
    id: 5,
    name: "LG French Door Refrigerator".to_string(),
    price: 1899.99,
    description: "Spacious and energy-efficient refrigerator with smart cooling technology and sleek stainless steel design.".to_string(),
    image: "/fridge.jpg".to_string()
},
Product {
    id: 6,
    name: "Bose Smart Soundbar".to_string(),
    price: 499.99,
    description: "Enhance your home theater experience with rich, immersive sound and built-in voice control.".to_string(),
    image: "/soundbar.jpg".to_string()
},
Product {
    id: 7,
    name: "Canon EOS Rebel T8i DSLR Camera".to_string(),
    price: 899.99,
    description: "Capture stunning photos and videos with this beginner-friendly DSLR camera featuring fast autofocus and 4K video.".to_string(),
    image: "/camera.jpg".to_string()
},
Product {
    id: 8,
    name: "PlayStation 5 Console".to_string(),
    price: 499.99,
    description: "Next-generation gaming console delivering lightning-fast load times, immersive graphics, and exclusive titles.".to_string(),
    image: "/ps5.jpg".to_string()
},
Product {
    id: 9,
    name: "Apple Watch Series 9".to_string(),
    price: 429.99,
    description: "Stay connected and track your fitness with advanced health features and a sleek, customizable design.".to_string(),
    image: "/watch.jpg".to_string()
},
Product {
    id: 10,
    name: "iRobot Roomba Robot Vacuum".to_string(),
    price: 349.99,
    description: "Smart robot vacuum that cleans your floors automatically, featuring intelligent navigation and app control.".to_string(),
    image: "/roomba.jpg".to_string()
}

    ]
}