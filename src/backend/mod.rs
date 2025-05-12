pub use audio_generation_backend::MusicGenJobProcessor;
pub use server::*;

pub use music_gpt_ws_handler::MusicGPTWebSocketHandler; 

mod audio_generation_backend;
mod server;
#[cfg(test)]
mod _test_utils;
mod music_gpt_chat;
mod audio_generation_fanout;
mod ws_handler;
mod music_gpt_ws_handler;

#[cfg(test)]
mod tests {
    use super::audio_generation_backend::MusicGenJobProcessor;
    use crate::backend::music_gpt_ws_handler::MusicGPTWebSocketHandler;

    #[ignore]
    #[tokio::test]
    async fn spawn_dummy_server() -> anyhow::Result<()> {
       
    }

   
    #[ignore]
    #[tokio::test]
    async fn test_music_gen_processor() -> anyhow::Result<()> {
        let storage = AppFs::new(Path::new("/tmp/music-gen-test"));
        let processor = MusicGenJobProcessor::new(Duration::from_secs(1));
        let options = RunOptions {
            port: 8643,
            auto_open: false,
            expose: false,
        };
        
        let test_handler = MusicGPTWebSocketHandler::new(processor.clone());
        let handle = tokio::spawn(run(storage, processor, options));
        
        handle.abort(); 
        Ok(())
    }
   
    #[ignore]
    #[test]
    fn export_bindings() -> anyhow::Result<()> {
       
    }
