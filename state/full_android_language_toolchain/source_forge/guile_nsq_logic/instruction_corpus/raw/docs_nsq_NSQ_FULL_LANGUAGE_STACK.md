# NSQ Full Language Stack

The full language stack is built into NSQ runtime sight.

Languages, dialects, shells, build tools, platform APIs, file formats, artifact formats, graphics APIs, boot surfaces, package managers, and ASM dialects are runtime surfaces.

They are not plugins.

They are not foreign masters.

They are not the truth under NSQ.

They are surfaces that NSQ can see, route, parse, lint, pack, inspect, and project.

A surface is present only when declared in the runtime registry.

A dialect is enabled only when the registry says it is enabled.

Insertion fails closed when the dialect is absent.

The full stack includes ordinary programming languages, Lisp-family symbolic languages, scripting languages, shell languages, markup languages, data/schema languages, database/query languages, package/build languages, Android/NDK/JNI/SDK surfaces, graphics/render surfaces, boot/platform surfaces, filesystem/artifact surfaces, model/runtime manifest surfaces, and ASM/IR surfaces.

The purpose is not to make NSQ a wrapper around those languages.

The purpose is to make those languages visible to the NSQ runtime court as native addressable surfaces.
