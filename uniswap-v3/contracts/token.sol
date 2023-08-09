// SPDX-License-Identifier: MIT
pragma solidity ^0.7.0;

import {Ownable, ERC20, ERC20Burnable, Address} from "./openzeppelin.sol";

contract ERC20Mintable is Ownable, ERC20Burnable {
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) {}

    function mint(address to, uint256 amount) onlyOwner external {
        _mint(to, amount);
    }
}

contract WETH is Ownable, ERC20Burnable {
    using Address for address payable;

    constructor() ERC20("Wrapped Ether", "WETH") {}

    function deposit() public payable {
        _mint(_msgSender(), msg.value);
    }

    function withdraw(uint256 amount) external {
        _burn(_msgSender(), amount);
    }

    receive() external payable {
        deposit();
    }
}