# P2P File Sharing

A simple peer-to-peer (P2P) file-sharing application in Rust.

## How to Run

### Prerequisites

Make sure you have Rust and Cargo installed on your machine.

- [Install Rust](https://www.rust-lang.org/learn/get-started)

### Steps to Run

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/p2p_file_sharing.git
   cd p2p_file_sharing
   ```

2. Build the project:

	```bash
	cargo build --release```

3. Run the server:

	```bash
	cargo run
	```

The server will be listening on 127.0.0.1:8080.

Open multiple terminals or PowerShell/CMD sessions for testing clients.

Commands to test (PowerShell Windows):

	```powershell
	$stream = [System.Net.Sockets.TcpClient]::new('127.0.0.1', 8080).GetStream()

	$writer = [System.IO.StreamWriter]::new($stream)
	$writer.WriteLine("Hello, P2P Server!")
	$writer.Flush()

	$reader = [System.IO.StreamReader]::new($stream)
	$response = $reader.ReadLine()
	$response
	```

Supported Operating Systems

- Windows
- Linux
- MacOS