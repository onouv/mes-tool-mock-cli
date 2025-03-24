

#[cfg(test)]
mod transitions {
    use actix::{Actor, Addr};

    use crate::tool::{LoadPartCmd, Part, StartCmd, StopCmd, Tool, ToolState};
    
    #[actix::test]
    async fn stopped_is_default() {
        let tool = Tool::new("1").start();
        

    }

    #[actix::test]
    #[ignore]
    async fn unloading_stopped_does_nothing() {
        assert_eq!(1, 0);
    }

    #[actix::test]
    async fn start_stopped_moves_to_idle() {
        let tool: Addr<Tool> = setup(ToolState::STOPPED).await;
        let snapshot = tool.send(StartCmd).await.unwrap();
        assert_eq!(snapshot.state, ToolState::IDLE);
    }

    #[actix::test]
    async fn loading_idle_moves_to_running() {
        let tool: Addr<Tool> = setup(ToolState::IDLE).await;
        let snapshot = tool.send(LoadPartCmd{ part: Part::new("1", "123")}).await.unwrap();
        assert_eq!(snapshot.state, ToolState::RUNNING);
        assert_eq!(snapshot.parts_inport, 0);
        assert_eq!(snapshot.parts_outport, 0);
        assert_eq!(snapshot.process_occupied, true);
        assert_eq!(snapshot.process_completion, None);
    }

    #[actix::test]
    #[ignore]
    async fn stop_running_moves_to_stopping() {
        assert_eq!(1, 0);
    }

    #[actix::test]
    #[ignore]
    async fn stop_idle_moves_to_stopped() {
        let tool: Addr<Tool> = setup(ToolState::IDLE).await;
        let snapshot = tool.send(StopCmd).await.unwrap();
        assert_eq!(snapshot.state, ToolState::STOPPED);
    }

    async fn setup(target_state: ToolState) -> Addr<Tool> {

        let tool = Tool::new("1").start();
        match target_state {
            
            ToolState::IDLE => {
                tool.send(StartCmd).await.unwrap();
            },
            ToolState::RUNNING => {
                tool.send(StartCmd).await.unwrap();
                tool.send(LoadPartCmd{ 
                    part: Part::new("1", "123")
                }).await.unwrap();
            },
            _ => {}
        }

        tool
    } 
}