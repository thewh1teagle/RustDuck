// Once the user logged in to duckdns.org, retrive token and domains

setInterval(() => {
  const user_items = document.querySelectorAll(".static-data > dl > dd");
  const token = user_items?.[2]?.innerText;
  const email = user_items?.[0]?.innerText;
  if (token) {
    const domains_tr = document.querySelectorAll("#domainsTable tbody tr");
    const domains = [];
    for (const tr of domains_tr) {
      const firstColumn = tr.querySelector("td");
      const domainName = firstColumn.innerText;
      domains.push({ name: domainName });
    }
    console.log("domains", domains);
    const config = { token, domains, email };
    window.__TAURI__.core.invoke("success_auth", { config });
  }
}, 1000);
