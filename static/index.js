const sleep = duration => new Promise(resolve => setTimeout(resolve, duration));

const poll = (promiseFn, duration) => {
    promiseFn().then(sleep(duration).then(() => poll(promiseFn, duration)));
}

const retrieve = async (event) => {
    const response = await fetch("/reports");
    const reports = await response.json();

    const reportsEl = document.getElementById("reports");
    reportsEl.innerText = "";

    reports.forEach(report => {
        const [name, ts] = report.replace('.csv','').split('-');
        const date = new Date(ts * 1000);
        const entry = document.createElement('li');
        const anchor = document.createElement('a');
        anchor.appendChild(document.createTextNode(`${name} ${date.toISOString()}`));
        anchor.setAttribute("href",`report/${report}`);
        entry.appendChild(anchor);
        reportsEl.appendChild(entry);
    });
};

addEventListener("load", () => {
    poll(() => new Promise(() => {
        retrieve();
    }), 10000);
});

const dragOverHandler = (event) => {
    event.preventDefault();
    event.target.classList.add("dragged");
}

const dropHandler = (event) => {
    event.preventDefault();

    const fileInput = document.getElementById("file");
    fileInput.files = event.dataTransfer.files;
    event.target.classList.remove("dragged");
}

const dragLeaveHandler = (event) => {
    event.preventDefault();
    event.target.classList.remove("dragged");
}
