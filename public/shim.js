switch (window.location.pathname) {
  case "/bakery" || "/bakery/order":
    window.dispatchEvent(
      new Event("rs-load", {
        bubbles: true,
        cancellable: true,
      })
    );
  case "/gym":
    window.dispatchEvent(
      new Event("rs-load-gym", {
        bubbles: true,
        cancellable: true,
      })
    );
}
