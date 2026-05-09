# NSQ Embedded Server Surface

First embedded server surface: SSHD.

Purpose:

The phone/Termux environment can expose a controlled local server entrance for Braxon/NSQ work.

Commands:

- Braxon-sshd-start
- Braxon-sshd-stop
- Braxon-sshd-status

Activation:

source tools/nsq/server/activate_embedded_server_surface.sh

Default port:

8022

Rule:

SSHD is a declared transport/control surface. It is not the whole runtime and it is not an undeclared backdoor. It is pinned, logged, and surfaced through explicit commands.
