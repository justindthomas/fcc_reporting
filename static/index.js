const httpRequest = new XMLHttpRequest();

httpRequest.onreadystatechange = (event) => {
    if (httpRequest.readyState === XMLHttpRequest.DONE) {
        const response = JSON.parse(httpRequest.responseText);

        const reports = document.getElementById("reports");
        reports.innerText = "";

        response.forEach(report => {
            let pruned = report.replace('.csv','');
            let parts = pruned.split('-');
            let date = new Date(parts[1] * 1000);
            let entry = document.createElement('li');
            let anchor = document.createElement('a');
            anchor.appendChild(document.createTextNode(`${parts[0]} ${date.toISOString()}`));
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
