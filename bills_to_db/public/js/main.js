function highlightMissingBills() {
    const expenseRows = document.querySelectorAll('tr.expense-row');

    expenseRows.forEach(row => {
        const billInput = row.querySelector('input.input-bill');

        if (billInput && parseFloat(billInput.value) === 0) {
            // Set background color to light red
            row.style.backgroundColor = '#ffcccc'; // light red
        }
    });
}

highlightMissingBills()


function countingExpenses() {
    const expenseRows = document.querySelectorAll('tr.expense-row');

    // reset totals & details
    document.getElementById('bmkos-amount').textContent = '0';
    document.getElementById('ma7-amount').textContent   = '0';
    document.getElementById('bezirk-amount').textContent= '0';

    let bundTotal   = 0, bundDetails   = [];
    let ma7Total    = 0, ma7Details    = [];
    let bezirkTotal = 0, bezirkDetails = [];

    expenseRows.forEach(row => {
        const appInput = Number(row.querySelector('#expense-application').value);
        const rowAmount = parseFloat(row.querySelector('.expense-amount').textContent) || 0;
        const typeText  = row.querySelector('#expense_type option:checked').textContent;

        if (appInput === 1) {
            bundTotal += rowAmount;
            bundDetails.push([typeText, rowAmount]);
        }
        else if (appInput === 2) {
            ma7Total += rowAmount;
            ma7Details.push([typeText, rowAmount]);
        }
        else if (appInput === 3) {
            bezirkTotal += rowAmount;
            bezirkDetails.push([typeText, rowAmount]);
        }
    });

    // write totals
    document.getElementById('bmkos-amount').textContent   = bundTotal.toFixed(2);
    document.getElementById('ma7-amount').textContent     = ma7Total.toFixed(2);
    document.getElementById('bezirk-amount').textContent  = bezirkTotal.toFixed(2);

    // helper to populate one table
    function fillTable(tableId, details) {
        const tbody = document.querySelector(`#${tableId} tbody`);
        tbody.innerHTML = '';  // clear existing
        details.forEach(([type, amt]) => {
            const tr = document.createElement('tr');
            tr.innerHTML = `<td>${type}</td><td>${amt.toFixed(2)}</td>`;
            tbody.appendChild(tr);
        });
    }

    // populate breakdown tables
    fillTable('bmkos-table',   bundDetails);
    fillTable('ma7-table',     ma7Details);
    fillTable('bezirk-table',  bezirkDetails);
}


countingExpenses();

function updateBillNumber(route, expenseId, newNumber) {
    sendingRequest(route, expenseId, newNumber)
    countingExpenses()
    console.log('update called')
}

function sendingRequest(route, expenseId, newNumber) {
    console.log(expenseId, newNumber)
    fetch(route, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            expense_id: expenseId,
            new_number: Number(newNumber)
        })
    })
        .then(response => response.json())
        .then(data => console.log('Success:', data))
        .catch(error => console.error('Error:', error))
}


function lockTableValues() {
    
}
