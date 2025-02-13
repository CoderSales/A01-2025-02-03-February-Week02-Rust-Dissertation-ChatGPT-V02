# ChatGPT

## Output Reduce Noise Capture Duration to 2 Seconds

Step 5: Reduce Noise Capture Duration to 2 Seconds
🔧 Change in fft.rs
Modify the noise profile capture duration from 5 seconds → 2 seconds for faster processing.

✅ Update:
rust
Copy
Edit
println!("Capturing noise for 2 seconds...");
std::thread::sleep(std::time::Duration::from_secs(2));
println!("Noise profile captured.");
🚀 Next Steps
1️⃣ Run cargo run again
2️⃣ Confirm if reducing capture time still allows proper sound detection
3️⃣ If successful, proceed with adding prompts for amplitude and frequency checks

Let me know the results! 🎙️
