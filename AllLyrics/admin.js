document.addEventListener('DOMContentLoaded', function() {
  const addUserForm = document.querySelector('#add-user-form');
  addUserForm.addEventListener('submit', function(event) {
    event.preventDefault(); // Prevent the form from submitting via the browser
    const formData = new FormData(addUserForm); // Serialize form data
    fetch('http://localhost:12345/users', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(Object.fromEntries(formData.entries()))
    })
    .then(response => {
      if (response.ok) {
        $('#addUserModal').modal('hide'); // Hide the modal after successful submission
        location.reload(); // Reload the page to see the changes
      } else {
        throw new Error('Error in response.');
      }
    })
    .catch(error => console.error(error));
  });
});

 document.addEventListener('DOMContentLoaded', function() {
  const deleteButton = document.querySelector('#deleteButton');
  deleteButton.addEventListener('click', function(event) {
    event.preventDefault();
    const checkedRows = document.querySelectorAll('.delete-checkbox:checked');
    if (checkedRows.length > 0) {
      const emails = Array.from(checkedRows).map(row => row.closest('tr').querySelector('.email').textContent);
      const formData = new FormData();
      formData.append('emails', JSON.stringify(emails));
      fetch('http://localhost:12345/users/{}',emails, {
        method: 'DELETE',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(Object.fromEntries(formData.entries()))
      })
      .then(response => {
        if (response.ok) {
          $('#deleteUserModal').modal('hide'); // Hide the modal after successful submission
          location.reload(); // Reload the page to see the changes
        } else {
          throw new Error('Error in response.');
        }
      })
      .catch(error => console.error(error));
    } else {
      alert('Please select at least one user to delete.');
    }
  });
});

   document.addEventListener('DOMContentLoaded', function() {
  fetch('http://localhost:12345/users')
    .then(response => {
      if (response.ok) {
        return response.json();
      } else {
        throw new Error('Error in response.');
      }
    })
    .then(data => {
      data.forEach((user, index) => {
        const row = document.createElement('tr');
        const id = index + 1; 
        row.innerHTML = `
          <td>${id}</td>
          <td>${user.name}</td>
          <td class="email" id="deleteUserEmail">${user.email}</td>
          <td>${user.role}</td>
          <td>${user.password}</td>
           <td><input type="checkbox" class="delete-checkbox"></td>
          
        `;
        document.querySelector('#user-table-body').appendChild(row);
      });
    })
    .catch(error => console.error(error));
});