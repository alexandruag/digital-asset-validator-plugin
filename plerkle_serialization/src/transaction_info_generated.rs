// automatically generated by the FlatBuffers compiler, do not modify

extern crate flatbuffers;

#[allow(unused_imports, dead_code)]
pub mod transaction_info {

    use core::cmp::Ordering;
    use core::mem;

    extern crate flatbuffers;
    use self::flatbuffers::{EndianScalar, Follow};

    pub enum TransactionInfoOffset {}
    #[derive(Copy, Clone, PartialEq)]

    pub struct TransactionInfo<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for TransactionInfo<'a> {
        type Inner = TransactionInfo<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table { buf, loc },
            }
        }
    }

    impl<'a> TransactionInfo<'a> {
        pub const VT_IS_VOTE: flatbuffers::VOffsetT = 4;
        pub const VT_ACCOUNT_KEYS: flatbuffers::VOffsetT = 6;
        pub const VT_LOG_MESSAGES: flatbuffers::VOffsetT = 8;
        pub const VT_INNER_INSTRUCTIONS: flatbuffers::VOffsetT = 10;
        pub const VT_OUTER_INSTRUCTIONS: flatbuffers::VOffsetT = 12;
        pub const VT_SLOT: flatbuffers::VOffsetT = 14;

        #[inline]
        pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            TransactionInfo { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args TransactionInfoArgs<'args>,
        ) -> flatbuffers::WIPOffset<TransactionInfo<'bldr>> {
            let mut builder = TransactionInfoBuilder::new(_fbb);
            builder.add_slot(args.slot);
            if let Some(x) = args.outer_instructions {
                builder.add_outer_instructions(x);
            }
            if let Some(x) = args.inner_instructions {
                builder.add_inner_instructions(x);
            }
            if let Some(x) = args.log_messages {
                builder.add_log_messages(x);
            }
            if let Some(x) = args.account_keys {
                builder.add_account_keys(x);
            }
            builder.add_is_vote(args.is_vote);
            builder.finish()
        }

        #[inline]
        pub fn is_vote(&self) -> bool {
            self._tab
                .get::<bool>(TransactionInfo::VT_IS_VOTE, Some(false))
                .unwrap()
        }
        #[inline]
        pub fn account_keys(
            &self,
        ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Pubkey<'a>>>> {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Pubkey>>,
            >>(TransactionInfo::VT_ACCOUNT_KEYS, None)
        }
        #[inline]
        pub fn log_messages(
            &self,
        ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>,
            >>(TransactionInfo::VT_LOG_MESSAGES, None)
        }
        #[inline]
        pub fn inner_instructions(
            &self,
        ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<InnerInstructions<'a>>>>
        {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<InnerInstructions>>,
            >>(TransactionInfo::VT_INNER_INSTRUCTIONS, None)
        }
        #[inline]
        pub fn outer_instructions(
            &self,
        ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInstruction<'a>>>>
        {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInstruction>>,
            >>(TransactionInfo::VT_OUTER_INSTRUCTIONS, None)
        }
        #[inline]
        pub fn slot(&self) -> u64 {
            self._tab
                .get::<u64>(TransactionInfo::VT_SLOT, Some(0))
                .unwrap()
        }
    }

    impl flatbuffers::Verifiable for TransactionInfo<'_> {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.visit_table(pos)?
                .visit_field::<bool>("is_vote", Self::VT_IS_VOTE, false)?
                .visit_field::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Pubkey>>,
                >>("account_keys", Self::VT_ACCOUNT_KEYS, false)?
                .visit_field::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<&'_ str>>,
                >>("log_messages", Self::VT_LOG_MESSAGES, false)?
                .visit_field::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<InnerInstructions>>,
                >>("inner_instructions", Self::VT_INNER_INSTRUCTIONS, false)?
                .visit_field::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<CompiledInstruction>>,
                >>("outer_instructions", Self::VT_OUTER_INSTRUCTIONS, false)?
                .visit_field::<u64>("slot", Self::VT_SLOT, false)?
                .finish();
            Ok(())
        }
    }
    pub struct TransactionInfoArgs<'a> {
        pub is_vote: bool,
        pub account_keys: Option<
            flatbuffers::WIPOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Pubkey<'a>>>,
            >,
        >,
        pub log_messages: Option<
            flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>,
        >,
        pub inner_instructions: Option<
            flatbuffers::WIPOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<InnerInstructions<'a>>>,
            >,
        >,
        pub outer_instructions: Option<
            flatbuffers::WIPOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInstruction<'a>>>,
            >,
        >,
        pub slot: u64,
    }
    impl<'a> Default for TransactionInfoArgs<'a> {
        #[inline]
        fn default() -> Self {
            TransactionInfoArgs {
                is_vote: false,
                account_keys: None,
                log_messages: None,
                inner_instructions: None,
                outer_instructions: None,
                slot: 0,
            }
        }
    }

    pub struct TransactionInfoBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> TransactionInfoBuilder<'a, 'b> {
        #[inline]
        pub fn add_is_vote(&mut self, is_vote: bool) {
            self.fbb_
                .push_slot::<bool>(TransactionInfo::VT_IS_VOTE, is_vote, false);
        }
        #[inline]
        pub fn add_account_keys(
            &mut self,
            account_keys: flatbuffers::WIPOffset<
                flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<Pubkey<'b>>>,
            >,
        ) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                TransactionInfo::VT_ACCOUNT_KEYS,
                account_keys,
            );
        }
        #[inline]
        pub fn add_log_messages(
            &mut self,
            log_messages: flatbuffers::WIPOffset<
                flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<&'b str>>,
            >,
        ) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                TransactionInfo::VT_LOG_MESSAGES,
                log_messages,
            );
        }
        #[inline]
        pub fn add_inner_instructions(
            &mut self,
            inner_instructions: flatbuffers::WIPOffset<
                flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<InnerInstructions<'b>>>,
            >,
        ) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                TransactionInfo::VT_INNER_INSTRUCTIONS,
                inner_instructions,
            );
        }
        #[inline]
        pub fn add_outer_instructions(
            &mut self,
            outer_instructions: flatbuffers::WIPOffset<
                flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<CompiledInstruction<'b>>>,
            >,
        ) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                TransactionInfo::VT_OUTER_INSTRUCTIONS,
                outer_instructions,
            );
        }
        #[inline]
        pub fn add_slot(&mut self, slot: u64) {
            self.fbb_
                .push_slot::<u64>(TransactionInfo::VT_SLOT, slot, 0);
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        ) -> TransactionInfoBuilder<'a, 'b> {
            let start = _fbb.start_table();
            TransactionInfoBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<TransactionInfo<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    impl core::fmt::Debug for TransactionInfo<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut ds = f.debug_struct("TransactionInfo");
            ds.field("is_vote", &self.is_vote());
            ds.field("account_keys", &self.account_keys());
            ds.field("log_messages", &self.log_messages());
            ds.field("inner_instructions", &self.inner_instructions());
            ds.field("outer_instructions", &self.outer_instructions());
            ds.field("slot", &self.slot());
            ds.finish()
        }
    }
    pub enum PubkeyOffset {}
    #[derive(Copy, Clone, PartialEq)]

    pub struct Pubkey<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for Pubkey<'a> {
        type Inner = Pubkey<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table { buf, loc },
            }
        }
    }

    impl<'a> Pubkey<'a> {
        pub const VT_KEY: flatbuffers::VOffsetT = 4;

        #[inline]
        pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            Pubkey { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args PubkeyArgs<'args>,
        ) -> flatbuffers::WIPOffset<Pubkey<'bldr>> {
            let mut builder = PubkeyBuilder::new(_fbb);
            if let Some(x) = args.key {
                builder.add_key(x);
            }
            builder.finish()
        }

        #[inline]
        pub fn key(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    Pubkey::VT_KEY,
                    None,
                )
                .map(|v| v.safe_slice())
        }
    }

    impl flatbuffers::Verifiable for Pubkey<'_> {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.visit_table(pos)?
                .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                    "key",
                    Self::VT_KEY,
                    false,
                )?
                .finish();
            Ok(())
        }
    }
    pub struct PubkeyArgs<'a> {
        pub key: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    }
    impl<'a> Default for PubkeyArgs<'a> {
        #[inline]
        fn default() -> Self {
            PubkeyArgs { key: None }
        }
    }

    pub struct PubkeyBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> PubkeyBuilder<'a, 'b> {
        #[inline]
        pub fn add_key(&mut self, key: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Pubkey::VT_KEY, key);
        }
        #[inline]
        pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PubkeyBuilder<'a, 'b> {
            let start = _fbb.start_table();
            PubkeyBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<Pubkey<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    impl core::fmt::Debug for Pubkey<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut ds = f.debug_struct("Pubkey");
            ds.field("key", &self.key());
            ds.finish()
        }
    }
    pub enum CompiledInstructionOffset {}
    #[derive(Copy, Clone, PartialEq)]

    pub struct CompiledInstruction<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for CompiledInstruction<'a> {
        type Inner = CompiledInstruction<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table { buf, loc },
            }
        }
    }

    impl<'a> CompiledInstruction<'a> {
        pub const VT_PROGRAM_ID_INDEX: flatbuffers::VOffsetT = 4;
        pub const VT_ACCOUNTS: flatbuffers::VOffsetT = 6;
        pub const VT_DATA: flatbuffers::VOffsetT = 8;

        #[inline]
        pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            CompiledInstruction { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args CompiledInstructionArgs<'args>,
        ) -> flatbuffers::WIPOffset<CompiledInstruction<'bldr>> {
            let mut builder = CompiledInstructionBuilder::new(_fbb);
            if let Some(x) = args.data {
                builder.add_data(x);
            }
            if let Some(x) = args.accounts {
                builder.add_accounts(x);
            }
            builder.add_program_id_index(args.program_id_index);
            builder.finish()
        }

        #[inline]
        pub fn program_id_index(&self) -> u8 {
            self._tab
                .get::<u8>(CompiledInstruction::VT_PROGRAM_ID_INDEX, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn accounts(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    CompiledInstruction::VT_ACCOUNTS,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn data(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    CompiledInstruction::VT_DATA,
                    None,
                )
                .map(|v| v.safe_slice())
        }
    }

    impl flatbuffers::Verifiable for CompiledInstruction<'_> {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.visit_table(pos)?
                .visit_field::<u8>("program_id_index", Self::VT_PROGRAM_ID_INDEX, false)?
                .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                    "accounts",
                    Self::VT_ACCOUNTS,
                    false,
                )?
                .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(
                    "data",
                    Self::VT_DATA,
                    false,
                )?
                .finish();
            Ok(())
        }
    }
    pub struct CompiledInstructionArgs<'a> {
        pub program_id_index: u8,
        pub accounts: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
        pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    }
    impl<'a> Default for CompiledInstructionArgs<'a> {
        #[inline]
        fn default() -> Self {
            CompiledInstructionArgs {
                program_id_index: 0,
                accounts: None,
                data: None,
            }
        }
    }

    pub struct CompiledInstructionBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> CompiledInstructionBuilder<'a, 'b> {
        #[inline]
        pub fn add_program_id_index(&mut self, program_id_index: u8) {
            self.fbb_.push_slot::<u8>(
                CompiledInstruction::VT_PROGRAM_ID_INDEX,
                program_id_index,
                0,
            );
        }
        #[inline]
        pub fn add_accounts(
            &mut self,
            accounts: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>,
        ) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                CompiledInstruction::VT_ACCOUNTS,
                accounts,
            );
        }
        #[inline]
        pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(CompiledInstruction::VT_DATA, data);
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        ) -> CompiledInstructionBuilder<'a, 'b> {
            let start = _fbb.start_table();
            CompiledInstructionBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<CompiledInstruction<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    impl core::fmt::Debug for CompiledInstruction<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut ds = f.debug_struct("CompiledInstruction");
            ds.field("program_id_index", &self.program_id_index());
            ds.field("accounts", &self.accounts());
            ds.field("data", &self.data());
            ds.finish()
        }
    }
    pub enum InnerInstructionsOffset {}
    #[derive(Copy, Clone, PartialEq)]

    pub struct InnerInstructions<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for InnerInstructions<'a> {
        type Inner = InnerInstructions<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table { buf, loc },
            }
        }
    }

    impl<'a> InnerInstructions<'a> {
        pub const VT_INDEX: flatbuffers::VOffsetT = 4;
        pub const VT_INSTRUCTIONS: flatbuffers::VOffsetT = 6;

        #[inline]
        pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            InnerInstructions { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args InnerInstructionsArgs<'args>,
        ) -> flatbuffers::WIPOffset<InnerInstructions<'bldr>> {
            let mut builder = InnerInstructionsBuilder::new(_fbb);
            if let Some(x) = args.instructions {
                builder.add_instructions(x);
            }
            builder.add_index(args.index);
            builder.finish()
        }

        #[inline]
        pub fn index(&self) -> u8 {
            self._tab
                .get::<u8>(InnerInstructions::VT_INDEX, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn instructions(
            &self,
        ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInstruction<'a>>>>
        {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInstruction>>,
            >>(InnerInstructions::VT_INSTRUCTIONS, None)
        }
    }

    impl flatbuffers::Verifiable for InnerInstructions<'_> {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.visit_table(pos)?
                .visit_field::<u8>("index", Self::VT_INDEX, false)?
                .visit_field::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<CompiledInstruction>>,
                >>("instructions", Self::VT_INSTRUCTIONS, false)?
                .finish();
            Ok(())
        }
    }
    pub struct InnerInstructionsArgs<'a> {
        pub index: u8,
        pub instructions: Option<
            flatbuffers::WIPOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<CompiledInstruction<'a>>>,
            >,
        >,
    }
    impl<'a> Default for InnerInstructionsArgs<'a> {
        #[inline]
        fn default() -> Self {
            InnerInstructionsArgs {
                index: 0,
                instructions: None,
            }
        }
    }

    pub struct InnerInstructionsBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> InnerInstructionsBuilder<'a, 'b> {
        #[inline]
        pub fn add_index(&mut self, index: u8) {
            self.fbb_
                .push_slot::<u8>(InnerInstructions::VT_INDEX, index, 0);
        }
        #[inline]
        pub fn add_instructions(
            &mut self,
            instructions: flatbuffers::WIPOffset<
                flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<CompiledInstruction<'b>>>,
            >,
        ) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                InnerInstructions::VT_INSTRUCTIONS,
                instructions,
            );
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        ) -> InnerInstructionsBuilder<'a, 'b> {
            let start = _fbb.start_table();
            InnerInstructionsBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<InnerInstructions<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    impl core::fmt::Debug for InnerInstructions<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut ds = f.debug_struct("InnerInstructions");
            ds.field("index", &self.index());
            ds.field("instructions", &self.instructions());
            ds.finish()
        }
    }
    #[inline]
    #[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
    pub fn get_root_as_transaction_info<'a>(buf: &'a [u8]) -> TransactionInfo<'a> {
        unsafe { flatbuffers::root_unchecked::<TransactionInfo<'a>>(buf) }
    }

    #[inline]
    #[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
    pub fn get_size_prefixed_root_as_transaction_info<'a>(buf: &'a [u8]) -> TransactionInfo<'a> {
        unsafe { flatbuffers::size_prefixed_root_unchecked::<TransactionInfo<'a>>(buf) }
    }

    #[inline]
    /// Verifies that a buffer of bytes contains a `TransactionInfo`
    /// and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_transaction_info_unchecked`.
    pub fn root_as_transaction_info(
        buf: &[u8],
    ) -> Result<TransactionInfo, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::root::<TransactionInfo>(buf)
    }
    #[inline]
    /// Verifies that a buffer of bytes contains a size prefixed
    /// `TransactionInfo` and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `size_prefixed_root_as_transaction_info_unchecked`.
    pub fn size_prefixed_root_as_transaction_info(
        buf: &[u8],
    ) -> Result<TransactionInfo, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::size_prefixed_root::<TransactionInfo>(buf)
    }
    #[inline]
    /// Verifies, with the given options, that a buffer of bytes
    /// contains a `TransactionInfo` and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_transaction_info_unchecked`.
    pub fn root_as_transaction_info_with_opts<'b, 'o>(
        opts: &'o flatbuffers::VerifierOptions,
        buf: &'b [u8],
    ) -> Result<TransactionInfo<'b>, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::root_with_opts::<TransactionInfo<'b>>(opts, buf)
    }
    #[inline]
    /// Verifies, with the given verifier options, that a buffer of
    /// bytes contains a size prefixed `TransactionInfo` and returns
    /// it. Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_transaction_info_unchecked`.
    pub fn size_prefixed_root_as_transaction_info_with_opts<'b, 'o>(
        opts: &'o flatbuffers::VerifierOptions,
        buf: &'b [u8],
    ) -> Result<TransactionInfo<'b>, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::size_prefixed_root_with_opts::<TransactionInfo<'b>>(opts, buf)
    }
    #[inline]
    /// Assumes, without verification, that a buffer of bytes contains a TransactionInfo and returns it.
    /// # Safety
    /// Callers must trust the given bytes do indeed contain a valid `TransactionInfo`.
    pub unsafe fn root_as_transaction_info_unchecked(buf: &[u8]) -> TransactionInfo {
        flatbuffers::root_unchecked::<TransactionInfo>(buf)
    }
    #[inline]
    /// Assumes, without verification, that a buffer of bytes contains a size prefixed TransactionInfo and returns it.
    /// # Safety
    /// Callers must trust the given bytes do indeed contain a valid size prefixed `TransactionInfo`.
    pub unsafe fn size_prefixed_root_as_transaction_info_unchecked(buf: &[u8]) -> TransactionInfo {
        flatbuffers::size_prefixed_root_unchecked::<TransactionInfo>(buf)
    }
    #[inline]
    pub fn finish_transaction_info_buffer<'a, 'b>(
        fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        root: flatbuffers::WIPOffset<TransactionInfo<'a>>,
    ) {
        fbb.finish(root, None);
    }

    #[inline]
    pub fn finish_size_prefixed_transaction_info_buffer<'a, 'b>(
        fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        root: flatbuffers::WIPOffset<TransactionInfo<'a>>,
    ) {
        fbb.finish_size_prefixed(root, None);
    }
} // pub mod TransactionInfo
