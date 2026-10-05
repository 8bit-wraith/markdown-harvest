use markdown_harvest::{HttpConfig, MarkdownHarvester};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

fn main() {
    println!("🦀 Markdown Harvest - Interactive CLI");
    println!("=====================================");
    println!();
    println!("Welcome to Markdown Harvest! Choose from the following options:");
    println!();

    loop {
        display_menu();

        let choice = get_user_choice();

        match choice.as_str() {
            "1" => {
                println!("\n🔄 Starting Synchronous Processing...");
                println!("{}", "=".repeat(50));
                run_sync_example();
            }
            "2" => {
                println!("\n⚡ Starting Asynchronous Processing...");
                println!("{}", "=".repeat(50));
                if let Err(e) = run_async_example_blocking() {
                    println!("❌ Error: {}", e);
                }
            }
            "3" => {
                #[cfg(feature = "chunks")]
                {
                    println!("\n📦 Starting Synchronous Chunking...");
                    println!("{}", "=".repeat(50));
                    run_sync_chunks_example();
                }
                #[cfg(not(feature = "chunks"))]
                {
                    println!("\n❌ Chunks feature not enabled!");
                    println!(
                        "To use chunking functionality, compile with: cargo run --features chunks"
                    );
                }
            }
            "4" => {
                #[cfg(feature = "chunks")]
                {
                    println!("\n🚀 Starting Asynchronous Chunking...");
                    println!("{}", "=".repeat(50));
                    if let Err(e) = run_async_chunks_example_blocking() {
                        println!("❌ Error: {}", e);
                    }
                }
                #[cfg(not(feature = "chunks"))]
                {
                    println!("\n❌ Chunks feature not enabled!");
                    println!(
                        "To use chunking functionality, compile with: cargo run --features chunks"
                    );
                }
            }
            "0" | "q" | "quit" | "exit" => {
                println!("👋 Goodbye! Thanks for using Markdown Harvest!");
                break;
            }
            _ => {
                println!("❌ Invalid choice! Please enter a number from 0-4.");
            }
        }

        println!();
        println!("Press Enter to continue...");
        let mut _dummy = String::new();
        io::stdin().read_line(&mut _dummy).ok();
    }
}

fn display_menu() {
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│                    🚀 Main Menu                        │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!("│ 1. 🔄 Synchronous Processing                           │");
    println!("│    Sequential URL processing - all results together    │");
    println!("│                                                         │");
    println!("│ 2. ⚡ Asynchronous Processing                          │");
    println!("│    Parallel URL processing - real-time results         │");
    println!("│                                                         │");
    println!("│ 3. 📦 Synchronous Chunking                            │");
    println!("│    Sequential processing with semantic chunking        │");
    println!("│                                                         │");
    println!("│ 4. 🚀 Asynchronous Chunking                           │");
    println!("│    Parallel processing with real-time chunking         │");
    println!("│                                                         │");
    println!("│ 0. 🚪 Exit                                             │");
    println!("└─────────────────────────────────────────────────────────┘");
    print!("Enter your choice (0-4): ");
    io::stdout().flush().unwrap();
}

fn get_user_choice() -> String {
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Error reading input");
    choice.trim().to_string()
}

fn get_user_input() -> String {
    println!();
    println!("Enter text containing URLs to extract content from:");
    println!("Example: Check out https://example.com and https://httpbin.org/json");
    println!("Or try: Visit https://www.rust-lang.org for more information");
    print!("Your text: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Error reading input");

    text
}

// Option 1: Synchronous Processing
fn run_sync_example() {
    println!("This example demonstrates the synchronous get_hyperlinks_content function.");
    println!("URLs will be processed sequentially, returning all results together.");

    let text = get_user_input();

    let http_config = HttpConfig::builder()
        .timeout(30000) // 30 second timeout
        .max_redirect(3) // Allow up to 3 redirects
        .cookie_store(true) // Enable cookie storage
        .build();

    println!("\n🔄 Processing URLs synchronously...");
    println!("⏳ Please wait while all URLs are processed sequentially...");
    println!();

    let start_time = std::time::Instant::now();

    let results = MarkdownHarvester::get_hyperlinks_content(text, http_config);

    let duration = start_time.elapsed();

    display_sync_results(&results, duration);
}

// Option 2: Asynchronous Processing (blocking wrapper)
fn run_async_example_blocking() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_async_example())
}

// Option 2: Asynchronous Processing
async fn run_async_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("This example demonstrates the asynchronous get_hyperlinks_content_async function.");
    println!("URLs will be processed in parallel, with results streaming as they complete.");

    let text = get_user_input();

    let http_config = HttpConfig::builder()
        .timeout(30000) // 30 second timeout
        .max_redirect(3) // Allow up to 3 redirects
        .cookie_store(true) // Enable cookie storage
        .build();

    println!("\n⚡ Processing URLs asynchronously...");
    println!("🚀 URLs will be processed in parallel - results appear as they complete:");
    println!();

    let start_time = std::time::Instant::now();
    let processed_count = Arc::new(Mutex::new(0));
    let processed_count_clone = processed_count.clone();

    let callback = move |url: Option<String>, content: Option<String>| {
        let processed_count = processed_count_clone.clone();
        async move {
            match (url, content) {
                (Some(url), Some(content)) => {
                    let mut count = processed_count.lock().unwrap();
                    *count += 1;
                    let current_count = *count;
                    drop(count);

                    println!("✅ Result #{}: {}", current_count, url);

                    let preview = if content.chars().count() > 200 {
                        let truncated: String = content.chars().take(150).collect();
                        format!(
                            "{}...\n\n📏 [Content truncated - {} total characters]",
                            truncated,
                            content.chars().count()
                        )
                    } else {
                        content.clone()
                    };

                    println!("📝 Markdown content:");
                    println!("{}", preview);
                    println!("{}", "─".repeat(60));
                    println!();
                }
                (None, None) => {
                    println!("ℹ️  No URLs found in the provided text");
                    println!("💡 Try entering text with URLs like: https://example.com");
                    println!();
                }
                _ => unreachable!(),
            }
        }
    };

    MarkdownHarvester::get_hyperlinks_content_async(text, http_config, callback).await?;

    let duration = start_time.elapsed();
    let final_count = *processed_count.lock().unwrap();

    println!(
        "⏱️  Asynchronous processing completed in {:.2}ms",
        duration.as_millis()
    );
    println!("📊 Total URLs processed: {}", final_count);
    println!("✅ Asynchronous processing example completed!");
    println!();
    println!("💡 Key characteristics of asynchronous processing:");
    println!("   • URLs are processed concurrently in parallel");
    println!("   • Results stream in real-time as each URL completes");
    println!("   • Better performance for multiple URLs");
    println!("   • Ideal for real-time applications and high throughput");

    Ok(())
}

// Option 3: Synchronous Chunking
#[cfg(feature = "chunks")]
fn run_sync_chunks_example() {
    println!(
        "This example demonstrates the synchronous get_hyperlinks_content_as_chunks function."
    );
    println!("URLs will be processed and content split into semantic chunks for RAG systems.");

    let text = get_user_input();
    let (chunk_size, chunk_overlap) = get_chunk_config();

    let http_config = HttpConfig::builder()
        .timeout(30000) // 30 second timeout
        .max_redirect(3) // Allow up to 3 redirects
        .cookie_store(true) // Enable cookie storage
        .build();

    println!("\n🔄 Processing URLs and creating chunks synchronously...");
    println!("📦 Chunk size: {} characters", chunk_size);
    if let Some(overlap) = chunk_overlap {
        println!("🔗 Chunk overlap: {} characters", overlap);
    } else {
        println!("🔗 No chunk overlap");
    }
    println!("⏳ Please wait while content is processed and chunked...");
    println!();

    let start_time = std::time::Instant::now();

    let results = MarkdownHarvester::get_hyperlinks_content_as_chunks(
        text,
        http_config,
        chunk_size,
        chunk_overlap,
    );

    let duration = start_time.elapsed();

    display_chunks_results(&results, duration, chunk_size, chunk_overlap);
}

// Option 4: Asynchronous Chunking (blocking wrapper)
#[cfg(feature = "chunks")]
fn run_async_chunks_example_blocking() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_async_chunks_example())
}

// Option 4: Asynchronous Chunking
#[cfg(feature = "chunks")]
async fn run_async_chunks_example() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "This example demonstrates the asynchronous get_hyperlinks_content_as_chunks_async function."
    );
    println!(
        "URLs will be processed in parallel and content split into semantic chunks for RAG systems."
    );

    let text = get_user_input();
    let (chunk_size, chunk_overlap) = get_chunk_config();

    let http_config = HttpConfig::builder()
        .timeout(30000) // 30 second timeout
        .max_redirect(3) // Allow up to 3 redirects
        .cookie_store(true) // Enable cookie storage
        .build();

    println!("\n⚡ Processing URLs and creating chunks asynchronously...");
    println!("📦 Chunk size: {} characters", chunk_size);
    if let Some(overlap) = chunk_overlap {
        println!("🔗 Chunk overlap: {} characters", overlap);
    } else {
        println!("🔗 No chunk overlap");
    }
    println!("🚀 URLs will be processed in parallel - chunks appear as they complete:");
    println!();

    let start_time = std::time::Instant::now();
    let processed_count = Arc::new(Mutex::new(0));
    let total_chunks = Arc::new(Mutex::new(0));
    let processed_count_clone = processed_count.clone();
    let total_chunks_clone = total_chunks.clone();

    let callback = move |url: Option<String>, chunks: Option<Vec<String>>| {
        let processed_count = processed_count_clone.clone();
        let total_chunks = total_chunks_clone.clone();
        async move {
            match (url, chunks) {
                (Some(url), Some(chunks)) => {
                    let mut count = processed_count.lock().unwrap();
                    *count += 1;
                    let current_count = *count;
                    drop(count);

                    let mut total = total_chunks.lock().unwrap();
                    *total += chunks.len();
                    let current_total = *total;
                    drop(total);

                    println!("✅ Result #{}: {}", current_count, url);
                    println!(
                        "📦 Chunks created: {} (Total so far: {})",
                        chunks.len(),
                        current_total
                    );
                    println!();

                    for (chunk_idx, chunk) in chunks.iter().enumerate() {
                        println!("   📝 Chunk #{}: {} characters", chunk_idx + 1, chunk.len());

                        let preview = if chunk.chars().count() > 120 {
                            let truncated: String = chunk.chars().take(80).collect();
                            format!("{}...", truncated)
                        } else {
                            chunk.clone()
                        };

                        println!("   Content: {}", preview);
                        println!();
                    }

                    println!("{}", "─".repeat(80));
                    println!();
                }
                (None, None) => {
                    println!("ℹ️  No URLs found in the provided text");
                    println!("💡 Try entering text with URLs like: https://example.com");
                    println!();
                }
                _ => unreachable!(),
            }
        }
    };

    MarkdownHarvester::get_hyperlinks_content_as_chunks_async(
        text,
        http_config,
        chunk_size,
        chunk_overlap,
        callback,
    )
    .await?;

    let duration = start_time.elapsed();
    let final_count = *processed_count.lock().unwrap();
    let final_total_chunks = *total_chunks.lock().unwrap();

    println!(
        "⏱️  Asynchronous chunking completed in {:.2}ms",
        duration.as_millis()
    );
    println!("📊 Total URLs processed: {}", final_count);
    println!("📦 Total chunks created: {}", final_total_chunks);
    println!("✅ Asynchronous chunking example completed!");
    println!();
    println!("💡 Configuration used:");
    println!("   📦 Chunk size: {} characters", chunk_size);
    if let Some(overlap) = chunk_overlap {
        println!(
            "   🔗 Chunk overlap: {} characters ({:.1}%)",
            overlap,
            (overlap as f64 / chunk_size as f64) * 100.0
        );
    } else {
        println!("   🔗 No chunk overlap");
    }
    println!();
    println!("🚀 Async Chunking Benefits:");
    println!("   • Parallel processing for faster completion with multiple URLs");
    println!("   • Real-time streaming of chunks as they're created");
    println!("   • Immediate processing of chunks for RAG pipeline integration");
    println!("   • Optimal for high-throughput applications and real-time systems");

    Ok(())
}

#[cfg(feature = "chunks")]
fn get_chunk_config() -> (usize, Option<usize>) {
    println!();
    println!("📦 Chunk Configuration:");
    println!("Choose a chunk size (recommended sizes):");
    println!("1. 🤖 OpenAI ada-002: 1000 characters");
    println!("2. 🤖 OpenAI text-embedding-3-large: 1500 characters");
    println!("3. 🤖 Cohere embed-multilingual: 2000 characters");
    println!("4. 🤖 Custom size");
    print!("Enter your choice (1-4): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Error reading input");

    let chunk_size = match choice.trim() {
        "1" => 1000,
        "2" => 1500,
        "3" => 2000,
        "4" => {
            print!("Enter custom chunk size: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let mut size = String::new();
            io::stdin()
                .read_line(&mut size)
                .expect("Error reading input");
            size.trim().parse().unwrap_or(1000)
        }
        _ => {
            println!("Invalid choice, using default: 1000");
            1000
        }
    };

    println!("\n🔗 Chunk Overlap Configuration:");
    println!("Do you want chunk overlap for better context preservation?");
    println!("1. No overlap (faster processing)");
    println!("2. 10% overlap (recommended for most cases)");
    println!("3. 20% overlap (better context, larger storage)");
    println!("4. Custom overlap");
    print!("Enter your choice (1-4): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut overlap_choice = String::new();
    io::stdin()
        .read_line(&mut overlap_choice)
        .expect("Error reading input");

    let chunk_overlap = match overlap_choice.trim() {
        "1" => None,
        "2" => Some(chunk_size / 10), // 10% overlap
        "3" => Some(chunk_size / 5),  // 20% overlap
        "4" => {
            print!("Enter custom overlap size: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let mut overlap = String::new();
            io::stdin()
                .read_line(&mut overlap)
                .expect("Error reading input");
            overlap.trim().parse().ok()
        }
        _ => {
            println!("Invalid choice, using no overlap");
            None
        }
    };

    (chunk_size, chunk_overlap)
}

fn display_sync_results(results: &[(String, String)], duration: std::time::Duration) {
    println!(
        "⏱️  Synchronous processing completed in {:.2}ms",
        duration.as_millis()
    );
    println!("📊 Summary: {} URL(s) processed", results.len());
    println!();

    if results.is_empty() {
        println!("ℹ️  No URLs found in the provided text.");
        println!("💡 Try entering text with URLs like: https://example.com");
        return;
    }

    for (i, (url, content)) in results.iter().enumerate() {
        println!("📄 Result #{}: {}", i + 1, url);

        let preview = if content.chars().count() > 300 {
            let truncated: String = content.chars().take(200).collect();
            format!(
                "{}...\n\n[Content truncated - {} total characters]",
                truncated,
                content.chars().count()
            )
        } else {
            content.clone()
        };

        println!("📝 Markdown content:");
        println!("{}", preview);
        println!("{}", "─".repeat(60));
    }

    println!();
    println!("✅ Synchronous processing example completed!");
    println!();
    println!("💡 Key characteristics of synchronous processing:");
    println!("   • URLs are processed one by one in sequence");
    println!("   • All results are returned together after processing completes");
    println!("   • Simple and straightforward - good for basic use cases");
    println!("   • Memory efficient for smaller numbers of URLs");
}

#[cfg(feature = "chunks")]
fn display_chunks_results(
    results: &[(String, Vec<String>)],
    duration: std::time::Duration,
    chunk_size: usize,
    chunk_overlap: Option<usize>,
) {
    println!(
        "⏱️  Synchronous chunking completed in {:.2}ms",
        duration.as_millis()
    );
    println!("📊 Summary: {} URL(s) processed", results.len());

    let total_chunks: usize = results.iter().map(|(_, chunks)| chunks.len()).sum();
    println!("📦 Total chunks created: {}", total_chunks);
    println!();

    if results.is_empty() {
        println!("ℹ️  No URLs found in the provided text.");
        println!("💡 Try entering text with URLs like: https://example.com");
        return;
    }

    for (i, (url, chunks)) in results.iter().enumerate() {
        println!("📄 Result #{}: {}", i + 1, url);
        println!("📦 Chunks created: {}", chunks.len());
        println!();

        for (chunk_idx, chunk) in chunks.iter().enumerate() {
            println!("   📝 Chunk #{}: {} characters", chunk_idx + 1, chunk.len());

            let preview = if chunk.chars().count() > 150 {
                let truncated: String = chunk.chars().take(100).collect();
                format!("{}...", truncated)
            } else {
                chunk.clone()
            };

            println!("   Content: {}", preview);
            println!();
        }

        println!("{}", "─".repeat(80));
    }

    println!();
    println!("✅ Synchronous chunking example completed!");
    println!();
    println!("💡 Configuration used:");
    println!("   📦 Chunk size: {} characters", chunk_size);
    if let Some(overlap) = chunk_overlap {
        println!(
            "   🔗 Chunk overlap: {} characters ({:.1}%)",
            overlap,
            (overlap as f64 / chunk_size as f64) * 100.0
        );
    } else {
        println!("   🔗 No chunk overlap");
    }
    println!();
    println!("🧠 RAG System Benefits:");
    println!("   • Content is semantically split preserving document structure");
    println!("   • Chunks maintain coherent meaning for better embeddings");
    println!("   • Optimal size for embedding models and vector databases");
    if chunk_overlap.is_some() {
        println!("   • Overlap ensures context continuity between chunks");
    }
}
