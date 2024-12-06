//! # Módulo `packet`
//!
//! Este módulo define estruturas e funções para manipulação de pacotes personalizados,
//! incluindo serialização, desserialização, e verificação de integridade através de checksum.
//!
//! ## Estruturas
//! - [`PacketHeader`]: Representa o cabeçalho de um pacote.
//! - [`Packet`]: Representa um pacote completo, incluindo o cabeçalho e o payload.
//!
//! ## Exemplos
//! ### Criação de um novo pacote
//! ```rust
//! use packet::Packet;
//!
//! let payload = vec![1, 2, 3, 4, 5];
//! let packet = Packet::new(1, 42, 12345, payload);
//! ```
//!
//! ### Serialização e Desserialização
//! ```rust
//! use packet::Packet;
//!
//! let payload = vec![1, 2, 3, 4, 5];
//! let packet = Packet::new(1, 42, 12345, payload.clone());
//! let bytes = packet.to_bytes();
//! let decoded = Packet::from_bytes(&bytes).unwrap();
//!
//! assert_eq!(packet.header, decoded.header);
//! assert_eq!(packet.payload, decoded.payload);
//! ```

/// Representa o cabeçalho de um pacote.
///
/// O cabeçalho contém informações básicas sobre o pacote, como tipo de mensagem,
/// sequência, ID do jogador, tamanho do payload e checksum.
#[derive(Debug)]
struct PacketHeader {
    /// Tipo de mensagem.
    message_type: u8,
    /// Sequência do pacote.
    sequence: u32,
    /// Identificador único do jogador.
    player_id: u64,
    /// Tamanho do payload em bytes.
    payload_size: u32,
    /// Checksum para controle de integridade.
    checksum: u32,
}

impl PacketHeader {
    /// Serializa o cabeçalho para um vetor de bytes.
    ///
    /// ## Retorno
    /// - `Vec<u8>`: Um vetor de bytes representando o cabeçalho.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(self.message_type);
        buffer.extend(&self.sequence.to_le_bytes());
        buffer.extend(&self.player_id.to_le_bytes());
        buffer.extend(&self.payload_size.to_le_bytes());
        buffer.extend(&self.checksum.to_le_bytes());
        buffer
    }

    /// Reconstrói o cabeçalho a partir de um vetor de bytes.
    ///
    /// ## Parâmetros
    /// - `bytes`: Fatia de bytes representando o cabeçalho.
    ///
    /// ## Retorno
    /// - `Result<Self, String>`: Retorna o cabeçalho em caso de sucesso ou uma mensagem de erro.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 21 {
            return Err("Bytes insuficientes para um cabeçalho".into());
        }

        let message_type = bytes[0];
        let sequence = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
        let player_id = u64::from_le_bytes(bytes[5..13].try_into().unwrap());
        let payload_size = u32::from_le_bytes(bytes[13..17].try_into().unwrap());
        let checksum = u32::from_le_bytes(bytes[17..21].try_into().unwrap());

        Ok(Self {
            message_type,
            sequence,
            player_id,
            payload_size,
            checksum,
        })
    }
}

/// Representa um pacote contendo um cabeçalho e um payload.
#[derive(Debug)]
struct Packet {
    /// Cabeçalho do pacote.
    header: PacketHeader,
    /// Dados do pacote.
    payload: Vec<u8>,
}

impl Packet {
    /// Serializa o pacote para um vetor de bytes.
    ///
    /// ## Retorno
    /// - `Vec<u8>`: Um vetor de bytes representando o pacote completo.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = self.header.to_bytes();
        buffer.extend(&self.payload);
        buffer
    }

    /// Reconstrói o pacote a partir de um vetor de bytes.
    ///
    /// ## Parâmetros
    /// - `bytes`: Fatia de bytes representando o pacote completo.
    ///
    /// ## Retorno
    /// - `Result<Self, String>`: Retorna o pacote em caso de sucesso ou uma mensagem de erro.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 21 {
            return Err("Bytes insuficientes para um pacote".into());
        }

        let header = PacketHeader::from_bytes(&bytes[..21])?;
        let payload = bytes[21..].to_vec();

        if payload.len() != header.payload_size as usize {
            return Err("Tamanho do payload não corresponde ao especificado no cabeçalho".into());
        }

        Ok(Self { header, payload })
    }

    /// Calcula o checksum de um payload.
    ///
    /// ## Parâmetros
    /// - `payload`: Referência para os bytes do payload.
    ///
    /// ## Retorno
    /// - `u32`: Soma dos valores dos bytes do payload.
    pub fn calculate_checksum(payload: &[u8]) -> u32 {
        payload.iter().map(|&b| b as u32).sum()
    }

    /// Cria um novo pacote com base nos parâmetros fornecidos.
    ///
    /// ## Parâmetros
    /// - `message_type`: Tipo da mensagem.
    /// - `sequence`: Sequência do pacote.
    /// - `player_id`: Identificador único do jogador.
    /// - `payload`: Dados do pacote.
    ///
    /// ## Retorno
    /// - `Self`: Um novo pacote.
    pub fn new(message_type: u8, sequence: u32, player_id: u64, payload: Vec<u8>) -> Self {
        let payload_size = payload.len() as u32;
        let checksum = Self::calculate_checksum(&payload);

        let header = PacketHeader {
            message_type,
            sequence,
            player_id,
            payload_size,
            checksum,
        };

        Self { header, payload }
    }
}
