cargo build --target x86_64-pc-windows-gnu --release
cargo build --release

rm -R final_build
mkdir final_build
cp -r assets final_build/assets
cp target/release/singleplayer_dungion_crawler final_build/singleplayer_dungion_crawler
cp target/x86_64-pc-windows-gnu/release/singleplayer_dungion_crawler.exe final_build/singleplayer_dungion_crawler.exe

rm game.zip 
zip -r game.zip final_build