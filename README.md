<div align="center">
  <img
    alt="Specter Logo"
    src="./img/specter.png"
  />
</div>

<br>

<div>
  <a href="https://specter-eth.neocities.org/" target="_blank" style="display: inline-block; margin-right: 20px;"><img src="https://img.shields.io/badge/Website-black" /></a>
  <a href="https://specter-eth.gitbook.io/specter" target="_blank" style="display: inline-block; margin-right: 20px;"><img src="https://img.shields.io/badge/Gitbook-black" /></a>
  <a href="https://twitter.com/eth_specter" target="_blank" style="display: inline-block;"><img src="https://img.shields.io/twitter/follow/eth_specter" /></a>
</div>

## Introducing Specter

For crypto to reach its next stage of growth, it must be used less as a means of speculation and
evolve to become more like money—a reliable medium of exchange and store of wealth.
If crypto is to reach a more money-like state, privacy is not just a nice-to-have. Everyone
should be able to receive payroll on-chain without revealing their salaries, make purchases
without divulging spending habits, and store their assets on-chain without disclosing their net
worth.
Despite the necessity, privacy adoption remains an unsolved problem. While the design space
of mixers, alternative chains, and semi-composable roll-ups have been explored, privacy still
remains a fringe category. To tackle this issue, we see two paths toward adoption: address the
unmet needs of the privacy-conscious and integrate privacy into existing applications for the
average user.

## Our Approach

Specter is a highly composable protocol for private accounts on Ethereum. Using a mix of
account abstraction and zero-knowledge proofs, we’ve built a private account layer that
enables users to send, receive, and transact with their funds, all without exposing their
address (check out our [Gitbook](https://specter-eth.gitbook.io/specter) to see how it works). It is designed as a generic system upon
which end-user products can be easily built. Given this flexibility, we can cater to both the
privacy maximalist as well as the traditional user.

<div align="center">
  <img src="./img/Anonymous Dex Swap.png" alt="Protocol flow example: Swapping ETH for DAI">
  <br>
  <sub style="font-size: larger; text-align: center;">Protocol flow example: Swapping ETH for DAI</sub>
</div>

## Private Vault

The simplest application we can build on Specter is a private vault that lets you store your
assets long-term, earn yield on idle assets, and withdraw to burner wallets for high-touch
activities like trading, all without exposing your address or net balances. 

## Backend for Private Payments

Instead of making payments directly to a recipient's EOA and exposing their financial history,
one can simply send funds into Specter and set their owner to the recipient’s stealth address.
There is already demand for this today in payroll for on-chain organizations—people don’t
want the whole world to know their salary. By integrating Specter into existing DAO payment
products, if you are a company administrator, keeping your employees’ salaries private will
be as simple as clicking a button.

## Integration with Tor

<p>
  <a href="https://www.torproject.org/" target="_blank">
    <img src="https://img.shields.io/badge/Tor_Project-purple" />
  </a>
</p>

By integrating Tor into your platform, in a **[Bisq](https://bisq.network/)** similar fashion, we further enhance user privacy and security by routing network communications through a decentralized network of volunteer-run servers. This integration ensures that users' IP addresses are masked, providing anonymity and resistance to network surveillance.

