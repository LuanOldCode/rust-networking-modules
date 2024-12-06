## Módulos

### 1. Módulo `packet`

Este módulo fornece estruturas e funções para a manipulação de pacotes personalizados, incluindo serialização, desserialização, e verificação de integridade através de checksum.

#### Estruturas
- [`PacketHeader`]: Representa o cabeçalho de um pacote, com informações sobre tipo de mensagem, sequência, ID do jogador, tamanho do payload e checksum.
- [`Packet`]: Representa um pacote completo, incluindo o cabeçalho e o payload.

#### Exemplos de Uso

##### Criação de um novo pacote

```rust
use packet::Packet;

let payload = vec![1, 2, 3, 4, 5];
let packet = Packet::new(1, 42, 12345, payload);
```

##### Serialização e Desserialização

```rust
use packet::Packet;

let payload = vec![1, 2, 3, 4, 5];
let packet = Packet::new(1, 42, 12345, payload.clone());
let bytes = packet.to_bytes();
let decoded = Packet::from_bytes(&bytes).unwrap();

assert_eq!(packet.header, decoded.header);
assert_eq!(packet.payload, decoded.payload);
```

### Estruturas do Módulo `packet`

#### `PacketHeader`

Representa o cabeçalho de um pacote de rede. Contém as informações básicas sobre o pacote, como:

- Tipo de mensagem
- Sequência do pacote
- Identificador do jogador
- Tamanho do payload
- Checksum para verificação de integridade

#### `Packet`

Representa o pacote completo, incluindo o cabeçalho (`PacketHeader`) e os dados (`payload`). Oferece métodos para:

- Serialização do pacote em um vetor de bytes
- Desserialização do pacote a partir de bytes
- Cálculo de checksum para garantir a integridade dos dados

### Como Usar

Adicione o módulo `packet` ao seu projeto para começar a manipular pacotes de rede de forma eficiente e segura.

## Como Contribuir

Sinta-se à vontade para abrir **issues** ou enviar **pull requests**. Estamos abertos a melhorias e sugestões!

---

Para mais informações, consulte os arquivos de implementação localizados no diretório `packet/packet.rs`.
