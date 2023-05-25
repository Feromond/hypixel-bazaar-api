# hypixel-bazaar-api
Uses hypixel-api to grab bazaar item product information in near-real-time and displays the data visually in the command line

<!-- TABLE OF CONTENTS -->

  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#release-notes">Release Notes</a>
      <ul>
        <li><a href="#cli-bazaar-product-fetch">cli-bazaar-product-fetch</a></li>
      </ul>
    </li>
    <li>
      <a href="#references">References</a>
    </li>
  </ol>

## About The Project

This project aims to provide information on bazaar item prices with frequent updates. Eventually I hope to include long-term persistent historical data which could be used for some ML experimentation and forecasting.
This is also serving as another Rust learning project for me and I am intending to use Rust for all, or almost all of the project.

![image](https://github.com/Feromond/hypixel-bazaar-api/assets/53460081/3b9db595-da15-41a5-af8c-7f9dc7027cdd)

## Release Notes:

### cli-bazaar-product-fetch
A general version of the booster cookie fetch. Now the user is prompted in the terminal to input a product id. This id is verified and if it exists in the hypixel api response, it will start providing the ASCII visualization and buy/sell price information of that item. It is a general tool to monitor the prices of different bazaar items (with a current 10 second set delay).

Future plans include storing historical data from specific items and providing that historical data when requesting that product id. This would provide much more insight since the current system only tracks prices starting at runtime and going until termination.

## References

[Hypixel Api](https://api.hypixel.net/)
