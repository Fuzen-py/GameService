[![Build Status](https://travis-ci.org/Mikibot/GameService.svg?branch=master)](https://travis-ci.org/Mikibot/GameService)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FFuzen-py%2FGameService.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2FFuzen-py%2FGameService?ref=badge_shield)

# NOTICE: The readme / API Examples are not up to date

## Install Requirements:
- [libpq](https://www.postgresql.org)
- [rust](https://rustup.rs)


## Setup:
Set the environment variables:

	GAMESERVICE_DATABASE_URL
	GAMESERVICE_TEST_DATABASE_URL

And configure [rocket](https://rocket.rs/guide/configuration/)

###### .env is supported, if you use it, you need to configure [Rocket.toml](https://rocket.rs/guide/configuration/#rockettoml)

## Routes:
- `/blackjack`
	- `GET: /` - Active Sessions (where game isnt in a completed state).
	- `GET: /<uid>` - Information about `<uid>`'s current game.
	- `POST: /<uid>/create/<bet>` - Creates a new game for `<uid>` with `<bet>` at stake.
	- `POST: /<uid>/hit` - Draw another card for `<uid>`.
	- `POST: /<uid>/stay` - Tell the dealer you are done, dealer will make its play.
	- `POST: /<uid>/claim` - Returns the amount bet + reward, either 0 or 2x the `bet`
- `/slot_machine`
	- `GET: /<bet>`
- `/coin_toss`
	- `GET: /<guess>/<bet>` - Valid guesses are `h/heads/t/tails`
- `/rps/`
	- `GET: /<weapon>/<bet>`

#### Sample Responses:
- [BlackJack](API/BlackJack.md)
- [SlotMachine](API/SlotMachine.md)
- [CoinToss](API/CoinToss.md)
- [RPS](API/RPS.md)

## License
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FFuzen-py%2FGameService.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2FFuzen-py%2FGameService?ref=badge_large)