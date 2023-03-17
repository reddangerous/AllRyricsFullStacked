import React from 'react';

const Header = () => {
  return (
    <header>
      <nav className="secondary-menu">
        <div className="container">
          <div className=" bg-cyan-500">
            <strong>Phone</strong>:&nbsp;<a href="#">+254 700 555 555</a>&nbsp;&nbsp;&nbsp;&nbsp;
            <strong>E-mail</strong>:&nbsp;<a href="#">music.allyrics.com</a>
          </div>
          <div className="sm-right">
            <div className="sm-social-link">
              <a className="h-facebook" href="#"><i className="fa fa-facebook"></i></a>
              <a className="h-twitter" href="#"><i className="fa fa-twitter"></i></a>
              <a className="h-google" href="#"><i className="fa fa-google-plus"></i></a>
              <a className="h-linkedin" href="#"><i className="fa fa-linkedin"></i></a>
            </div>
          </div>
          <div className="clearfix"></div>
        </div>
      </nav>
      <nav className="navbar navbar-fixed-top navbar-default">
        <div className="container">
          <div className="navbar-header">
            <button type="button" className="navbar-toggle collapsed" data-toggle="collapse" data-target="#bs-example-navbar-collapse-1">
              <span className="sr-only">Toggle navigation</span>
              <span className="icon-bar"></span>
              <span className="icon-bar"></span>
              <span className="icon-bar"></span>
            </button>
            <a className="navbar-brand" href="#home">
              <img className="img-responsive" src="img/logo/logo.png" alt="" />
              <span className="site-name">Allyrics</span>
            </a>
          </div>
          <div className="collapse navbar-collapse" id="bs-example-navbar-collapse-1">
            <ul className="nav navbar-nav navbar-right">
              <li><a href="#">Home</a></li>
              <li><a href="#">About</a></li>
              <li><a href="#">Music</a></li>
              <li><a href="#">Blog</a></li>
              <li><a href="#">Contact</a></li>
              <li><a href="#" data-toggle="modal" data-target="#bookTicket">Book Now</a></li>
            </ul>
          </div>
        </div>
      </nav>
    </header>
    );
}

export default Header;