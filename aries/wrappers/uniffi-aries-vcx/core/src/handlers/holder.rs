use std::sync::{Arc, Mutex};

use aries_vcx::{
    handlers::issuance::holder::Holder as VcxHolder, messages::AriesMessage,
    protocols::issuance::holder::state_machine::HolderState as VcxHolderState,
};

use crate::{core::profile::ProfileHolder, errors::error::VcxUniFFIResult, runtime::block_on};
pub struct Holder {
    handler: Mutex<VcxHolder>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HolderState {
    Initial,
    ProposalSet,
    OfferReceived,
    RequestSet,
    Finished,
    Failed,
}

impl From<VcxHolderState> for HolderState {
    fn from(x: VcxHolderState) -> Self {
        match x {
            VcxHolderState::Initial => HolderState::Initial,
            VcxHolderState::ProposalSet => HolderState::ProposalSet,
            VcxHolderState::OfferReceived => HolderState::OfferReceived,
            VcxHolderState::RequestSet => HolderState::RequestSet,
            VcxHolderState::Finished => HolderState::Finished,
            VcxHolderState::Failed => HolderState::Failed,
        }
    }
}

pub fn create(source_id: String) -> VcxUniFFIResult<Arc<Holder>> {
    block_on(async {
        let handler = Mutex::new(VcxHolder::create(&source_id)?);
        Ok(Arc::new(Holder { handler }))
    })
}

pub fn create_from_offer(source_id: String, offer_message: String) -> VcxUniFFIResult<Arc<Holder>> {
    let offer_message = serde_json::from_str(&offer_message)?;
    block_on(async {
        let handler = Mutex::new(VcxHolder::create_from_offer(&source_id, offer_message)?);
        Ok(Arc::new(Holder { handler }))
    })
}

pub fn create_with_proposal(
    source_id: String,
    propose_credential: String,
) -> VcxUniFFIResult<Arc<Holder>> {
    let propose_credential = serde_json::from_str(&propose_credential)?;

    block_on(async {
        let handler = Mutex::new(VcxHolder::create_with_proposal(
            &source_id,
            propose_credential,
        )?);
        Ok(Arc::new(Holder { handler }))
    })
}

impl Holder {
    pub fn set_proposal(&self, credential_proposal: String) -> VcxUniFFIResult<()> {
        let mut handler = self.handler.lock()?;

        handler.set_proposal(serde_json::from_str(&credential_proposal)?)?;
        Ok(())
    }

    pub fn prepare_credential_request(
        &self,
        profile: Arc<ProfileHolder>,
        my_pw_did: String,
    ) -> VcxUniFFIResult<()> {
        let mut handler = self.handler.lock()?;
        let mut holder = handler.clone();

        block_on(async {
            holder
                .prepare_credential_request(
                    profile.inner.wallet(),
                    profile.inner.ledger_read(),
                    profile.inner.anoncreds(),
                    my_pw_did,
                )
                .await?;
            *handler = holder;

            Ok(())
        })
    }

    pub fn get_msg_credential_request(&self) -> VcxUniFFIResult<String> {
        let handler = self.handler.lock()?;

        Ok(serde_json::to_string(&AriesMessage::from(
            handler.clone().get_msg_credential_request()?,
        ))?)
    }

    pub fn decline_offer(&self, comment: Option<String>) -> VcxUniFFIResult<String> {
        let mut handler = self.handler.lock()?;
        let mut holder = handler.clone();
        let problem_report_result = &holder.decline_offer(comment.as_deref())?;
        *handler = holder;

        Ok(serde_json::to_string(problem_report_result)?)
    }

    pub fn process_credential(
        &self,
        profile: Arc<ProfileHolder>,
        credential: String,
    ) -> VcxUniFFIResult<()> {
        let credential = serde_json::from_str(&credential)?;
        let mut handler = self.handler.lock()?;
        let mut holder = handler.clone();

        block_on(async {
            holder
                .process_credential(
                    profile.inner.wallet(),
                    profile.inner.ledger_read(),
                    profile.inner.anoncreds(),
                    credential,
                )
                .await?;
            *handler = holder;
            Ok(())
        })
    }

    pub fn is_terminal_state(&self) -> VcxUniFFIResult<bool> {
        let handler = self.handler.lock()?;

        Ok(handler.is_terminal_state())
    }

    pub fn get_state(&self) -> VcxUniFFIResult<HolderState> {
        let handler = self.handler.lock()?;

        Ok(HolderState::from(handler.get_state()))
    }

    pub fn get_source_id(&self) -> VcxUniFFIResult<String> {
        let handler = self.handler.lock()?;

        Ok(handler.get_source_id())
    }

    pub fn get_credential(&self) -> VcxUniFFIResult<String> {
        let handler = self.handler.lock()?;
        let credential = handler.get_credential()?;
        Ok(credential.0)
    }

    pub fn get_attributes(&self) -> VcxUniFFIResult<String> {
        let handler = self.handler.lock()?;

        Ok(handler.get_attributes()?)
    }

    pub fn get_attachment(&self) -> VcxUniFFIResult<String> {
        let handler = self.handler.lock()?;

        Ok(handler.get_attachment()?)
    }

    pub fn get_offer(&self) -> VcxUniFFIResult<String> {
        let handler = self.handler.lock()?;

        Ok(serde_json::to_string(&(handler.get_offer()?))?)
    }

    pub fn get_tails_location(&self) -> VcxUniFFIResult<String> {
        let handler = self.handler.lock()?;

        Ok(handler.get_tails_location()?)
    }

    pub fn get_tails_hash(&self) -> VcxUniFFIResult<String> {
        let handler = self.handler.lock()?;

        Ok(handler.get_tails_hash()?)
    }

    pub fn get_rev_reg_id(&self) -> VcxUniFFIResult<String> {
        let handler = self.handler.lock()?;

        Ok(handler.get_rev_reg_id()?)
    }

    pub fn get_cred_id(&self) -> VcxUniFFIResult<String> {
        let handler = self.handler.lock()?;

        Ok(handler.get_cred_id()?)
    }

    pub fn get_thread_id(&self) -> VcxUniFFIResult<String> {
        let handler = self.handler.lock()?;

        Ok(handler.get_thread_id()?)
    }

    pub fn is_revokable(&self, profile: Arc<ProfileHolder>) -> VcxUniFFIResult<bool> {
        let handler = self.handler.lock()?.clone();

        block_on(async { Ok(handler.is_revokable(profile.inner.ledger_read()).await?) })
    }

    pub fn is_revoked(&self, profile: Arc<ProfileHolder>) -> VcxUniFFIResult<bool> {
        let handler = self.handler.lock()?.clone();

        block_on(async {
            Ok(handler
                .is_revoked(
                    profile.inner.wallet(),
                    profile.inner.ledger_read(),
                    profile.inner.anoncreds(),
                )
                .await?)
        })
    }

    pub fn get_cred_rev_id(&self, profile: Arc<ProfileHolder>) -> VcxUniFFIResult<String> {
        let handler = self.handler.lock()?.clone();

        block_on(async {
            Ok(handler
                .get_cred_rev_id(profile.inner.wallet(), profile.inner.anoncreds())
                .await?)
        })
    }

    pub fn get_problem_report(&self) -> VcxUniFFIResult<String> {
        let handler = self.handler.lock()?;

        Ok(serde_json::to_string(&handler.get_problem_report()?)?)
    }

    pub fn get_final_message(&self) -> VcxUniFFIResult<Option<String>> {
        let handler = self.handler.lock()?;

        Ok(Some(serde_json::to_string(&handler.get_final_message()?)?))
    }
}
