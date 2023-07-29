const sleep = duration => new Promise(resolve => setTimeout(resolve, duration));

const poll = (promiseFn, duration) => {
    promiseFn().then(sleep(duration).then(() => poll(promiseFn, duration)));
}

const retrieve = async (event) => {
    fetch("/reports").then((response) => {
        response.json().then((reports) => {
            fetch("/statistics").then((response) => {
                response.json().then((statistics) => {

                    if (reports.length) {
                        const reportsEl = document.getElementById("reports");
                        reportsEl.innerText = '';
                        
                        const thead = document.createElement('thead');
                        reportsEl.appendChild(thead);
                        
                        const theadTr = document.createElement('tr');
                        thead.appendChild(theadTr);
                        
                        const typeHeading = document.createElement('th');
                        typeHeading.appendChild(document.createTextNode("Report Type"));
                        theadTr.appendChild(typeHeading);
                        
                        const timeHeading = document.createElement('th');
                        timeHeading.appendChild(document.createTextNode("Creation Timestamp"));
                        theadTr.appendChild(timeHeading);
                        
                        const tbody = document.createElement('tbody');
                        reportsEl.appendChild(tbody);

                        reports.forEach(report => {
                            const [name, ts] = report.replace('.csv','').split('-');
                            const date = new Date(ts * 1000);
                            const entry = document.createElement('tr');
                            
                            const nameCell = document.createElement('td');
                            const anchor = document.createElement('a');
                            anchor.appendChild(document.createTextNode(`${name}`));
                            anchor.setAttribute("href",`report/${report}`);
                            nameCell.appendChild(anchor);
                            
                            const timeCell = document.createElement('td');
                            timeCell.appendChild(document.createTextNode(`${date.toISOString()}`));
                            
                            entry.appendChild(nameCell);
                            entry.appendChild(timeCell);
                            
                            tbody.appendChild(entry);
                        });
                    }
                });
            });
        });
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

document.addEventListener('submit', (event) => {
    const form = event.target;

    fetch(form.action, {
        method: form.method,
        body: new FormData(form),
    }).then(() => {
        form.reset();
    })

    event.preventDefault();
});
