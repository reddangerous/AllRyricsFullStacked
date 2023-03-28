 document.addEventListener('DOMContentLoaded', function () {
  const signupForm = document.querySelector('#loginForm');
  signupForm.addEventListener('submit', function (event) {
    event.preventDefault(); // Prevent the form from submitting via the browser
    const email = document.getElementById('email')
    const password = document.getElementById('password') // Serialize form data
    fetch('http://localhost:1234/users/${email}', {
      method: 'POST',
      headers: {
        'content-Type': 'application/json',
      },
    })
      .then(response => {
      if (response.ok) {
        response.json().then(data => {
          console.log(data);
          if (data.role === 'admin') {
            window.location.href = 'admin.html'; // Redirect to admin dashboard for admin users
          } else {
            Swal.fire({
              title: 'Success!',
              text: 'Login successful!',
              icon: 'success',
              confirmButtonText: 'OK'
            }).then((result) => {
              if (result.isConfirmed) {
                window.location.href = 'index.html'; // Redirect to home page for non-admin users
              }
            });
          }
        });
      } else {
        throw new Error('Error in response.');
      }
    })
    .catch(error => {
      Swal.fire({
        title: 'Error!',
        text: error.message,
        icon: 'error',
        confirmButtonText: 'OK'
      });
    });
  });
});
