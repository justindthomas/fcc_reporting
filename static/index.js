const httpRequest = new XMLHttpRequest();

httpRequest.onreadystatechange = (event) => {
    if (httpRequest.readyState === XMLHttpRequest.DONE) {
        const response = JSON.parse(httpRequest.responseText);

        const reports = document.getElementById("reports");
        reports.innerText = "";

        response.forEach(report => {
            const [name, ts] = report.replace('.csv','').split('-');
            const date = new Date(ts * 1000);
            const entry = document.createElement('li');
            const anchor = document.createElement('a');
            anchor.appendChild(document.createTextNode(`${name} ${date.toISOString()}`));
            anchor.setAttribute("href",`report/${report}`);
            entry.appendChild(anchor);
            reports.appendChild(entry);
        });
    }
};

const sleep = duration => new Promise(resolve => setTimeout(resolve, duration));
const poll = (promiseFn, duration) => {
    promiseFn().then(sleep(duration).then(() => poll(promiseFn, duration)));
}

poll(() => new Promise(() => {
    httpRequest.open("GET", "/reports", true);
    httpRequest.send();
}), 10000);
