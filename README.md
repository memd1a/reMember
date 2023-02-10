# reMember

Somewhat like an emulator project to preserve an old mushroom(v95) game

## Setup

1. Install the latest stable rust compiler(state 10.02.2023 1.67)
2. Add the required keys the directory should look like this(ONLY net keys are required right now):
```keys/
├── data
│   ├── aes.bin
│   └── iv.bin
└── net
    ├── aes_key.bin
    ├── initial_round_key.bin
    └── round_shifting_key.bin
```
3. Build the project(`cargo b`)


## Structure

* packets: `net/moople_derive`, `net/moople_packet`
* net protocol: `net/moople_net`