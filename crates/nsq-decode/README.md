# nsq-decode

`nsq-decode` decodes derived packed artifacts back into readable symbol streams and report summaries. It is explicitly a derived decode lane and must not be treated as canonical NSQ semantic authority.

## Responsibilities
- read a derived JSON artifact containing packed symbols and transitions
- reconstruct readable symbol text for the `decode` mode
- emit structural counts and hash data for the `report` mode

## Command
```bash
cargo run -p nsq-decode --release -- decode <artifact>
cargo run -p nsq-decode --release -- report <artifact>
```

## Inputs and outputs
- Input: a derived packed JSON artifact
- Output: readable symbol text or a JSON decode report

## Workspace links
- Often paired with `nsq-real-bench`
- Useful after `nsq-preserve` or other derived-transport experiments
