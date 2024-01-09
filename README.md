# Rust UO Server

This is an implementation of an Ultima Online server I've been building whilst learning Rust. I'm using it as a playground for things I've learnt and want to put into practice, and also to work out how to tackle some hard problems in Rust.

[Ultima Online (UO)](https://en.wikipedia.org/wiki/Ultima_Online) is a fantasy massively multiplayer online role-playing game (MMORPG) originally released in 1997. There are still official servers run by [Broadsword](https://broadsword.com/about.html), running a closed-source implementation. There are also a large amount of free to play servers that run on open-source, community built server implementations such as this one which emulate the behaviour of the official servers. The most popular implementations are written in C#. As far as I'm aware there aren't any existing Rust implementations. I'm hoping that my implementation will provide at least some interesting benchmarks, if not a server implementation that is adopted by the community and prolongs the life of a game that I love.

I have been documenting my progress on my [public journal](https://thisdotrob.github.io/) - see all posts [tagged "UO server project"](https://thisdotrob.github.io/tag/UO%20server%20project/). They cover my thoughts on how I've architected the server, the design decisions and some code walkthroughs.

## Progress & next tasks

The following list obviously isn't a complete list of tasks, just those I've completed so far and the ones I think would be most interesting and useful to tackle next:

- [x] Basic timer logic ([1df6c58](https://github.com/thisdotrob/rust-uo-server/commit/1df6c58e504fb63577774237225872644ce8acc1))
- [x] Add CLI to trigger test timers ([dd9ab54](https://github.com/thisdotrob/rust-uo-server/commit/dd9ab54a64a5ecc19526d91d8f145b92450b4384))
- [x] Refactor timer logic ([3dc6e81](https://github.com/thisdotrob/rust-uo-server/commit/3dc6e8166ef937fcbf28f850121ad4c5806aa2e8), [a21b024](https://github.com/thisdotrob/rust-uo-server/commit/a21b024cccd5ab1026d730ddc87665e9779e1ddc), [ee83e96](https://github.com/thisdotrob/rust-uo-server/commit/ee83e96e1de2886a628a1e5aaa120f35d6263162), [66cd327](https://github.com/thisdotrob/rust-uo-server/commit/66cd3273062ced3afbfc94fbfef6075a1fddee38))
- [x] Explore approaches to adding callbacks to timers and making them multi-threaded ([fe86c53](https://github.com/thisdotrob/rust-uo-server/commit/fe86c531f12b7068aa7fa5783c50bba2685621f6), [161cd3c](https://github.com/thisdotrob/rust-uo-server/commit/161cd3c39edfef13a550c0031637d1b2068b4ae8), [25774cd](https://github.com/thisdotrob/rust-uo-server/commit/25774cd117a58d67ab5113b994598f2f67c8ee8c), [5553241](https://github.com/thisdotrob/rust-uo-server/commit/55532411640dce317cdbc89d56f0210879d50ca6))
- [x] Basic TCP connection handling ([a1744e4](https://github.com/thisdotrob/rust-uo-server/commit/a1744e4c280fa49f0ea6416487492108fd5bdc54), [95fdea7](https://github.com/thisdotrob/rust-uo-server/commit/95fdea728d4bf394689daccdd51af3bc1c92ed19), [44f855b](https://github.com/thisdotrob/rust-uo-server/commit/44f855bef63687c65412e0bd3b2e4ecd57e119dd), [7bbb24a](https://github.com/thisdotrob/rust-uo-server/commit/7bbb24a83095097ccd77e6decf068c8a08712b4b))
- [x] Receive, parse and respond to login and shard selection packets from game client ([a1744e4](https://github.com/thisdotrob/rust-uo-server/commit/a1744e4c280fa49f0ea6416487492108fd5bdc54), [9964692](https://github.com/thisdotrob/rust-uo-server/commit/996469216dea568bb687c2b4820b78618dd197ae), [d484f7c](https://github.com/thisdotrob/rust-uo-server/commit/d484f7cee2f852bb54580b0dc35ffc8607146287), [8f98387](https://github.com/thisdotrob/rust-uo-server/commit/8f983877331eaa87b61871ba36a8c92601c1e979))
- [x] Non-blocking connection handling to allow multiple clients ([98c3641](https://github.com/thisdotrob/rust-uo-server/commit/98c3641177aa20a9802b4dd63c103b8aa88e4336), [1af0e99](https://github.com/thisdotrob/rust-uo-server/commit/1af0e99931279eee34f5f785ec8a07f79ea883ac), [03042d8](https://github.com/thisdotrob/rust-uo-server/commit/03042d8f53cf05f9f65de433189a95c946ac05ef), [26970df](https://github.com/thisdotrob/rust-uo-server/commit/26970dfb04d6a48961532d4634cf508b3e023414))
- [x] Packet compression for "in-game" packets ([0b1e8f6](https://github.com/thisdotrob/rust-uo-server/commit/0b1e8f62eeb8d1a59d7cfc46e43bdcf4fec7de07))
- [x] Refactor compression interface and module structure ([08265ac](https://github.com/thisdotrob/rust-uo-server/commit/08265ac2812b2f9b6adeed20cb05df11ea761f08), [10a5f61](https://github.com/thisdotrob/rust-uo-server/commit/10a5f6117d73efb2917f55db34db91ac61138571))
- [ ] Refactor TCP module - extract sub modules and write tests ([acac0ed](https://github.com/thisdotrob/rust-uo-server/commit/acac0ed75cc2b89b9ca02f8eda6d9f2edd68901f))
- [ ] Finalise design of timer logic then add tests and documentation
- [ ] Store connection state and associate it with timers
- [ ] Receive, parse and respond to initial "in-game" packets
- [ ] Load game world state on server startup
- [ ] Save game world state to DB at intervals


