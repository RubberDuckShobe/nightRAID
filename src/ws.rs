use async_trait::async_trait;
use ezsockets::CloseFrame;
use ezsockets::Error;
use ezsockets::Request;
use ezsockets::Socket;
use std::net::SocketAddr;
use tracing::debug;
use tracing::error;

use crate::db;
use crate::game::commands;

type SessionID = u16;
type Session = ezsockets::Session<SessionID, ()>;

pub struct GameServer {
    pub pool: sqlx::PgPool,
}

pub struct GameSession {
    pub handle: Session,
    pub id: SessionID,
    pub user: Option<db::User>,
}

#[async_trait]
impl ezsockets::ServerExt for GameServer {
    type Session = GameSession;
    type Call = ();

    async fn on_connect(
        &mut self,
        socket: Socket,
        _request: Request,
        address: SocketAddr,
    ) -> Result<Session, Option<CloseFrame>> {
        let id = address.port();
        let session = Session::create(
            |handle| GameSession {
                id,
                handle,
                user: None,
            },
            id,
            socket,
        );
        let _ = session
            .text(
                "Welcome to nightRAID.\nPlease register using the \"register\" command.
If you are already registered, use the \"login\" command.
If you lost your access token, contact m1nt_.\n\n",
            )
            .map_err(|_| error!("Failed to send welcome message to {}", address));
        Ok(session)
    }

    async fn on_disconnect(
        &mut self,
        _id: <Self::Session as ezsockets::SessionExt>::ID,
        _reason: Result<Option<CloseFrame>, Error>,
    ) -> Result<(), Error> {
        Ok(())
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), Error> {
        let () = call;
        println!("server called");
        Ok(())
    }
}

#[async_trait]
impl ezsockets::SessionExt for GameSession {
    type ID = SessionID;
    type Call = ();

    fn id(&self) -> &Self::ID {
        &self.id
    }

    async fn on_text(&mut self, text: String) -> Result<(), Error> {
        let text = text.trim();
        debug!("received: {}", text);
        let command_result = commands::execute(self, text);
        match command_result {
            Ok(_response) => return Ok(()),
            Err(_error) => return Ok(()),
        };
    }

    async fn on_binary(&mut self, _bytes: Vec<u8>) -> Result<(), Error> {
        unimplemented!()
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), Error> {
        let () = call;
        self.handle.text("called")?;
        Ok(())
    }
}
