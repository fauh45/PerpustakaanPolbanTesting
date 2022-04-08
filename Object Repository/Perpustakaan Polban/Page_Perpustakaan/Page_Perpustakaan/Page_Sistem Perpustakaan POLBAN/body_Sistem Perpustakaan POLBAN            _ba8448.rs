<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Sistem Perpustakaan POLBAN            _ba8448</name>
   <tag></tag>
   <elementGuidId>d8d955ce-21e4-45c0-b7a3-40beb6bc820d</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body.hold-transition.login-page</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>hold-transition login-page</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>





  
  
  	
  		
  			Sistem Perpustakaan POLBAN
  			
  		
  	
    
      
      
	      
	      	User Name
	      
	      
	        
	      
       
      
      	
	  		Password
        
        
	      
      	
      
      
        
        
        
         LOGIN
        
        
         RESET
        
        
        
          FORGOT PASSWORD
        
        
      
      
        
        
        
          REGISTRASI
        
      
    
  
  































$(&quot;#username&quot;).focus();

var message = &quot;&quot;;
var code = &quot;&quot;;
if(message != null &amp;&amp; &quot;&quot; != message){
	showMessage(code, message);
}

$( &quot;#password&quot; ).keyup(function(event) {
	  if (event.keyCode === 13) {
		  doLogin();
	  }
});
	
function reset(){
	$('#username').val(&quot;&quot;);
	$('#password').val(&quot;&quot;);
	
}
function doForgot(){
	window.location=&quot;/PerpustakaanWebApp/forgotPassword&quot;;
}
function doRegistrasi(){
	window.location=&quot;/PerpustakaanWebApp/registrasi&quot;;
}
	
$(&quot;#forgot&quot;).click(function(){
	doForgot();
});

$(&quot;#btnLogin&quot;).click(function(){
	doLogin();
})
function doLogin(){
	if (doValidation()){
		document.getElementById(&quot;loginForm&quot;).submit();
	}
}

function doValidation(){
	var err = 0;
	if ($(&quot;#password&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Password should be filled in&quot;);
		err++;
	}
	if ($(&quot;#username&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field User Name should be filled in&quot;);
		err++;
	}
	
	if(err == 0){
		return true;
	}else{
		return false;
	}
	
}








</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;hold-transition login-page&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;





  
  
  	
  		
  			Sistem Perpustakaan POLBAN
  			
  		
  	
    
      
      
	      
	      	User Name
	      
	      
	        
	      
       
      
      	
	  		Password
        
        
	      
      	
      
      
        
        
        
         LOGIN
        
        
         RESET
        
        
        
          FORGOT PASSWORD
        
        
      
      
        
        
        
          REGISTRASI
        
      
    
  
  































$(&quot;#username&quot;).focus();

var message = &quot;&quot;;
var code = &quot;&quot;;
if(message != null &amp;&amp; &quot;&quot; != message){
	showMessage(code, message);
}

$( &quot;#password&quot; ).keyup(function(event) {
	  if (event.keyCode === 13) {
		  doLogin();
	  }
});
	
function reset(){
	$(&quot; , &quot;'&quot; , &quot;#username&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
	$(&quot; , &quot;'&quot; , &quot;#password&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
	
}
function doForgot(){
	window.location=&quot;/PerpustakaanWebApp/forgotPassword&quot;;
}
function doRegistrasi(){
	window.location=&quot;/PerpustakaanWebApp/registrasi&quot;;
}
	
$(&quot;#forgot&quot;).click(function(){
	doForgot();
});

$(&quot;#btnLogin&quot;).click(function(){
	doLogin();
})
function doLogin(){
	if (doValidation()){
		document.getElementById(&quot;loginForm&quot;).submit();
	}
}

function doValidation(){
	var err = 0;
	if ($(&quot;#password&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Password should be filled in&quot;);
		err++;
	}
	if ($(&quot;#username&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field User Name should be filled in&quot;);
		err++;
	}
	
	if(err == 0){
		return true;
	}else{
		return false;
	}
	
}








&quot;) or . = concat(&quot;





  
  
  	
  		
  			Sistem Perpustakaan POLBAN
  			
  		
  	
    
      
      
	      
	      	User Name
	      
	      
	        
	      
       
      
      	
	  		Password
        
        
	      
      	
      
      
        
        
        
         LOGIN
        
        
         RESET
        
        
        
          FORGOT PASSWORD
        
        
      
      
        
        
        
          REGISTRASI
        
      
    
  
  































$(&quot;#username&quot;).focus();

var message = &quot;&quot;;
var code = &quot;&quot;;
if(message != null &amp;&amp; &quot;&quot; != message){
	showMessage(code, message);
}

$( &quot;#password&quot; ).keyup(function(event) {
	  if (event.keyCode === 13) {
		  doLogin();
	  }
});
	
function reset(){
	$(&quot; , &quot;'&quot; , &quot;#username&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
	$(&quot; , &quot;'&quot; , &quot;#password&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
	
}
function doForgot(){
	window.location=&quot;/PerpustakaanWebApp/forgotPassword&quot;;
}
function doRegistrasi(){
	window.location=&quot;/PerpustakaanWebApp/registrasi&quot;;
}
	
$(&quot;#forgot&quot;).click(function(){
	doForgot();
});

$(&quot;#btnLogin&quot;).click(function(){
	doLogin();
})
function doLogin(){
	if (doValidation()){
		document.getElementById(&quot;loginForm&quot;).submit();
	}
}

function doValidation(){
	var err = 0;
	if ($(&quot;#password&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Password should be filled in&quot;);
		err++;
	}
	if ($(&quot;#username&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field User Name should be filled in&quot;);
		err++;
	}
	
	if(err == 0){
		return true;
	}else{
		return false;
	}
	
}








&quot;))]</value>
   </webElementXpaths>
</WebElementEntity>
