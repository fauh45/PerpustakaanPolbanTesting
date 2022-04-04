<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_POLBAN LIBRARY SYSTEM                  _1d888e</name>
   <tag></tag>
   <elementGuidId>4377c8b6-c6db-494e-92ed-7a57268e4dcd</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.wrapper</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[2]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>wrapper</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

  

    

    
      
      



      POLBAN LIBRARY SYSTEM
    

    

    
      



      
      
        
          
          




            
              
              
                
                
                  
                
              
              Tampilkan Semua
            
          
          
          
          
            
       			USER PROFILE


            
            
              






              
              
              
              
                   PROFILE
              
              
                   LOG OUT 
              
            
          
        
      

    
  
  
  
    
    
      
      
        
          
          ADMINISTRATOR
        



      
      

      

      	
          
             Dashboard
          
        
      
        
          
             User Management
          
        
      
      
        
        
        
      
    
    
  

  
  
    
  	












	
		
			
				FORM EDIT USER
			
		
		
		
			
				
					
					Name *
					
					
						
							 
						
					
				
				
					
					Email *
					
					
						
							 
						
					
				
				
					
					Role *
					
					
						
							-- Select One --ADMINISTRATORMEMBEROFFICIAL
						
					
				
				
					
							
							NIP *
						
						
							
								
							
						
					
					
						
						Staff *
						
						
							
								-- Select One--Head Of LibraryLibrarian StaffOffice Boy
							
						
					
				
				
					
						
						NIM *
						
						
							
								
							
						
					
					
						
						Majors *
						
						
							
								-- Select One--Teknik ElektroTeknik MesinTeknik SipilTeknik Konversi EnergiTeknik Refrigerasi dan Tata UdaraTeknik Komputer dan InformatikaTeknik KimiaAkuntansiAdministrasi Niaga
							
						
					
					
						
						Prodi *
						
						
							
								-- Select One--DIII Teknik ListrikDIII Teknik ElektronikaDIII Teknik TelekomumnikasiDIV Teknik ElektronikaDIV Teknik TelekomunikasiDIV Teknik Otomasi IndustriDIII Teknik AeronautikaDIII Teknik MesinDIII Teknologi MesinDIV Proses ManufakturDIV Teknik Perancangan dan Konstruksi MesinDIII Konstruksi SipilDIII Konsentrasi GedungDIV Teknik Perancangan Jalan dan JembatanDIV Teknik Perawatan dan Perbaikan GedungDIII Teknik konversi EnergiDIV Teknik Konversi EnergiDIV Teknologi Pembangkit Tenaga ListrikDIII Teknik Refrigerasi dan Tata UdaraDIV Teknik Refrigerasi dan Tata UdaraDIII Teknik InformatikaDIV Teknik InformatikaDIII Teknik KimiaDIII Analis KimiaDIV Teknik Kimia Produksi BersihDIII AkuntansiDIII Keuangan PerbankanDIV AkuntansiDIV Akuntansi Manajemen PemerintahanDIV Keuangan SyariahDIII Administrasi BisnisDIII Manajemen PemasaranDIII Administrasi BisnisDIV Administrasi BisnisDIV Manajemen PemasaranDIV Manajemen Aset
							
						
					
				
				
					
					
					
						
							SUBMIT
						
						
							RESET
						
						
							CANCEL
						
					
					
				
			
		
	





$(document).ready(function() {
	fillComboRole();
	fillComboMajors();
	fillComboProdi(&quot;&quot;);
	fillComboDivision();
	setTimeout(function(){ initialData(); }, 1000);	
});
$(&quot;#majorsId&quot;).change(function(){
	fillComboProdi(this.value);
})

$(&quot;#roleId&quot;).change(function(){
	if(this.value == &quot;OFC&quot;){
		$(&quot;#formMember&quot;).hide();
		$(&quot;#formOfficial&quot;).show();
	}else if(this.value == &quot;MBR&quot;){
		$(&quot;#formMember&quot;).show();
		$(&quot;#formOfficial&quot;).hide();
	}else{
		$(&quot;#formMember&quot;).hide();
		$(&quot;#formOfficial&quot;).hide();
	}
	
})

$(&quot;#btnSubmit&quot;).click(function(){
	if(doValidation()){
		doSave();
	}
})

function initialData(){
	$(&quot;#name&quot;).val(&quot;Fauzan Lubos&quot;);
	$(&quot;#email&quot;).val(&quot;fauzan@fauh45.my.id&quot;);
	$(&quot;#roleId&quot;).val(&quot;OFC&quot;);
	$(&quot;#nip&quot;).val(&quot;6942069&quot;);
	$(&quot;#divId&quot;).val(&quot;S2&quot;);
	$(&quot;#nim&quot;).val(&quot;&quot;);
	$(&quot;#majorsId&quot;).val(&quot;&quot;);
	$(&quot;#prodiId&quot;).val(&quot;&quot;);
	if (&quot;EDIT&quot; == &quot;ADD&quot;){
		$(&quot;#formMember&quot;).hide();
		$(&quot;#formOfficial&quot;).hide();
	}else{
		if($(&quot;#roleId&quot;).val() == &quot;OFC&quot;){
			$(&quot;#formMember&quot;).hide();
			$(&quot;#formOfficial&quot;).show();
		}else if($(&quot;#roleId&quot;).val() == &quot;MBR&quot;){
			$(&quot;#formMember&quot;).show();
			$(&quot;#formOfficial&quot;).hide();
		}else{
			$(&quot;#formMember&quot;).hide();
			$(&quot;#formOfficial&quot;).hide();
		}
	}
}
function doSave(){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/save&quot;,
        type: &quot;post&quot;,
        data: {	name:$(&quot;#name&quot;).val(),
        		email:$(&quot;#email&quot;).val(),
        		role:$(&quot;#roleId&quot;).val(),
        		nim:$(&quot;#nim&quot;).val(),
        		majors:$(&quot;#majorsId&quot;).val(),
        		prodi:$(&quot;#prodiId&quot;).val(),
        		nip:$(&quot;#nip&quot;).val(),
        		division:$(&quot;#divId&quot;).val(),
        		mode: &quot;EDIT&quot; },
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	if (response.status){
        		setTimeout(function(){ window.location=&quot;/PerpustakaanWebApp/user&quot;; }, 1000);	
        	}
        	showMessage(response.code, response.message);
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}
$(&quot;#btnReset&quot;).click(function(){
	initialData()
// 	$(&quot;#name&quot;).val(&quot;&quot;);
// 	$(&quot;#email&quot;).val(&quot;&quot;);
// 	$(&quot;#roleId&quot;).val(&quot;&quot;);
// 	$(&quot;#nip&quot;).val(&quot;&quot;);
// 	$(&quot;#divId&quot;).val(&quot;&quot;);
// 	$(&quot;#nim&quot;).val(&quot;&quot;);
// 	$(&quot;#majorsId&quot;).val(&quot;&quot;);
// 	$(&quot;#prodiId&quot;).val(&quot;&quot;);
// 	$(&quot;#formMember&quot;).hide();
// 	$(&quot;#formOfficial&quot;).hide();
})
$(&quot;#btnCancel&quot;).click(function(){
	window.location=&quot;/PerpustakaanWebApp/user&quot;;	
})

function doValidation(){
	var err = 0;
	
	if ($(&quot;#name&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Name should be filled in&quot;);
		err++;
	}
	if ($(&quot;#email&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Email should be filled in&quot;);
		err++;
	}
	if ($(&quot;#roleId&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Role should be filled in&quot;);
		err++;
	}
	if ($(&quot;#roleId&quot;).val() == &quot;OFC&quot;){
		if ($(&quot;#nip&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field NIP should be filled in&quot;);
			err++;
		}
		if ($(&quot;#divId&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Staff should be filled in&quot;);
			err++;
		}
	}else if ($(&quot;#roleId&quot;).val() == &quot;MBR&quot;){
		if ($(&quot;#nim&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field NIM should be filled in&quot;);
			err++;
		}
		if ($(&quot;#majorsId&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Majors should be filled in&quot;);
			err++;
		}
		if ($(&quot;#prodiId&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Prodi should be filled in&quot;);
			err++;
		}
	}
	if(err == 0){
		return true;
	}else{
		return false;
	}
	
}
function fillComboRole(){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/getComboRole&quot;,
        type: &quot;get&quot;,
        data: {	},
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	// fill combo Prodi
        	var data = response.data;
        	$('#roleId').empty()
        	$('#roleId').append('&lt;option value=&quot;&quot; selected=&quot;selected&quot;>-- Select One --&lt;/option>');
        	for(var i=0;  i&lt;data.length; i++){
        		$('#roleId').append('&lt;option value=&quot;'+data[i].value+'&quot; >'+data[i].label+'&lt;/option>');
        	}
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}

function fillComboProdi(majorsIdData){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/getComboProdi&quot;,
        type: &quot;get&quot;,
        data: {majorsId:majorsIdData},
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	// fill combo Prodi
        	var data = response.data;
        	$('#prodiId').empty()
        	$('#prodiId').append('&lt;option value=&quot;&quot; selected=&quot;selected&quot;>-- Select One--&lt;/option>');
        	for(var i=0;  i&lt;data.length; i++){
        		$('#prodiId').append('&lt;option value=&quot;'+data[i].value+'&quot; >'+data[i].label+'&lt;/option>');
        	}
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}

function fillComboMajors(){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/getComboJurusan&quot;,
        type: &quot;get&quot;,
        data: {	},
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	// fill combo Jurusan
        	var data = response.data;
        	$('#majorsId').empty()
        	$('#majorsId').append('&lt;option value=&quot;&quot; selected=&quot;selected&quot;>-- Select One--&lt;/option>');
        	for(var i=0;  i&lt;data.length; i++){
        		$('#majorsId').append('&lt;option value=&quot;'+data[i].value+'&quot; >'+data[i].label+'&lt;/option>');
        	}
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}

function fillComboDivision(){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/getComboDivision&quot;,
        type: &quot;get&quot;,
        data: {	},
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	// fill combo Jurusan
        	var data = response.data;
        	$('#divId').empty()
        	$('#divId').append('&lt;option value=&quot;&quot; selected=&quot;selected&quot;>-- Select One--&lt;/option>');
        	for(var i=0;  i&lt;data.length; i++){
        		$('#divId').append('&lt;option value=&quot;'+data[i].value+'&quot; >'+data[i].label+'&lt;/option>');
        	}
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}
	


    
  
  

  
    
      Politeknik Negeri Bandung LIBRARY 2020
    
  
</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;skin-red-light sidebar-mini&quot;]/div[@class=&quot;wrapper&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;

  

    

    
      
      



      POLBAN LIBRARY SYSTEM
    

    

    
      



      
      
        
          
          




            
              
              
                
                
                  
                
              
              Tampilkan Semua
            
          
          
          
          
            
       			USER PROFILE


            
            
              






              
              
              
              
                   PROFILE
              
              
                   LOG OUT 
              
            
          
        
      

    
  
  
  
    
    
      
      
        
          
          ADMINISTRATOR
        



      
      

      

      	
          
             Dashboard
          
        
      
        
          
             User Management
          
        
      
      
        
        
        
      
    
    
  

  
  
    
  	












	
		
			
				FORM EDIT USER
			
		
		
		
			
				
					
					Name *
					
					
						
							 
						
					
				
				
					
					Email *
					
					
						
							 
						
					
				
				
					
					Role *
					
					
						
							-- Select One --ADMINISTRATORMEMBEROFFICIAL
						
					
				
				
					
							
							NIP *
						
						
							
								
							
						
					
					
						
						Staff *
						
						
							
								-- Select One--Head Of LibraryLibrarian StaffOffice Boy
							
						
					
				
				
					
						
						NIM *
						
						
							
								
							
						
					
					
						
						Majors *
						
						
							
								-- Select One--Teknik ElektroTeknik MesinTeknik SipilTeknik Konversi EnergiTeknik Refrigerasi dan Tata UdaraTeknik Komputer dan InformatikaTeknik KimiaAkuntansiAdministrasi Niaga
							
						
					
					
						
						Prodi *
						
						
							
								-- Select One--DIII Teknik ListrikDIII Teknik ElektronikaDIII Teknik TelekomumnikasiDIV Teknik ElektronikaDIV Teknik TelekomunikasiDIV Teknik Otomasi IndustriDIII Teknik AeronautikaDIII Teknik MesinDIII Teknologi MesinDIV Proses ManufakturDIV Teknik Perancangan dan Konstruksi MesinDIII Konstruksi SipilDIII Konsentrasi GedungDIV Teknik Perancangan Jalan dan JembatanDIV Teknik Perawatan dan Perbaikan GedungDIII Teknik konversi EnergiDIV Teknik Konversi EnergiDIV Teknologi Pembangkit Tenaga ListrikDIII Teknik Refrigerasi dan Tata UdaraDIV Teknik Refrigerasi dan Tata UdaraDIII Teknik InformatikaDIV Teknik InformatikaDIII Teknik KimiaDIII Analis KimiaDIV Teknik Kimia Produksi BersihDIII AkuntansiDIII Keuangan PerbankanDIV AkuntansiDIV Akuntansi Manajemen PemerintahanDIV Keuangan SyariahDIII Administrasi BisnisDIII Manajemen PemasaranDIII Administrasi BisnisDIV Administrasi BisnisDIV Manajemen PemasaranDIV Manajemen Aset
							
						
					
				
				
					
					
					
						
							SUBMIT
						
						
							RESET
						
						
							CANCEL
						
					
					
				
			
		
	





$(document).ready(function() {
	fillComboRole();
	fillComboMajors();
	fillComboProdi(&quot;&quot;);
	fillComboDivision();
	setTimeout(function(){ initialData(); }, 1000);	
});
$(&quot;#majorsId&quot;).change(function(){
	fillComboProdi(this.value);
})

$(&quot;#roleId&quot;).change(function(){
	if(this.value == &quot;OFC&quot;){
		$(&quot;#formMember&quot;).hide();
		$(&quot;#formOfficial&quot;).show();
	}else if(this.value == &quot;MBR&quot;){
		$(&quot;#formMember&quot;).show();
		$(&quot;#formOfficial&quot;).hide();
	}else{
		$(&quot;#formMember&quot;).hide();
		$(&quot;#formOfficial&quot;).hide();
	}
	
})

$(&quot;#btnSubmit&quot;).click(function(){
	if(doValidation()){
		doSave();
	}
})

function initialData(){
	$(&quot;#name&quot;).val(&quot;Fauzan Lubos&quot;);
	$(&quot;#email&quot;).val(&quot;fauzan@fauh45.my.id&quot;);
	$(&quot;#roleId&quot;).val(&quot;OFC&quot;);
	$(&quot;#nip&quot;).val(&quot;6942069&quot;);
	$(&quot;#divId&quot;).val(&quot;S2&quot;);
	$(&quot;#nim&quot;).val(&quot;&quot;);
	$(&quot;#majorsId&quot;).val(&quot;&quot;);
	$(&quot;#prodiId&quot;).val(&quot;&quot;);
	if (&quot;EDIT&quot; == &quot;ADD&quot;){
		$(&quot;#formMember&quot;).hide();
		$(&quot;#formOfficial&quot;).hide();
	}else{
		if($(&quot;#roleId&quot;).val() == &quot;OFC&quot;){
			$(&quot;#formMember&quot;).hide();
			$(&quot;#formOfficial&quot;).show();
		}else if($(&quot;#roleId&quot;).val() == &quot;MBR&quot;){
			$(&quot;#formMember&quot;).show();
			$(&quot;#formOfficial&quot;).hide();
		}else{
			$(&quot;#formMember&quot;).hide();
			$(&quot;#formOfficial&quot;).hide();
		}
	}
}
function doSave(){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/save&quot;,
        type: &quot;post&quot;,
        data: {	name:$(&quot;#name&quot;).val(),
        		email:$(&quot;#email&quot;).val(),
        		role:$(&quot;#roleId&quot;).val(),
        		nim:$(&quot;#nim&quot;).val(),
        		majors:$(&quot;#majorsId&quot;).val(),
        		prodi:$(&quot;#prodiId&quot;).val(),
        		nip:$(&quot;#nip&quot;).val(),
        		division:$(&quot;#divId&quot;).val(),
        		mode: &quot;EDIT&quot; },
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	if (response.status){
        		setTimeout(function(){ window.location=&quot;/PerpustakaanWebApp/user&quot;; }, 1000);	
        	}
        	showMessage(response.code, response.message);
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}
$(&quot;#btnReset&quot;).click(function(){
	initialData()
// 	$(&quot;#name&quot;).val(&quot;&quot;);
// 	$(&quot;#email&quot;).val(&quot;&quot;);
// 	$(&quot;#roleId&quot;).val(&quot;&quot;);
// 	$(&quot;#nip&quot;).val(&quot;&quot;);
// 	$(&quot;#divId&quot;).val(&quot;&quot;);
// 	$(&quot;#nim&quot;).val(&quot;&quot;);
// 	$(&quot;#majorsId&quot;).val(&quot;&quot;);
// 	$(&quot;#prodiId&quot;).val(&quot;&quot;);
// 	$(&quot;#formMember&quot;).hide();
// 	$(&quot;#formOfficial&quot;).hide();
})
$(&quot;#btnCancel&quot;).click(function(){
	window.location=&quot;/PerpustakaanWebApp/user&quot;;	
})

function doValidation(){
	var err = 0;
	
	if ($(&quot;#name&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Name should be filled in&quot;);
		err++;
	}
	if ($(&quot;#email&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Email should be filled in&quot;);
		err++;
	}
	if ($(&quot;#roleId&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Role should be filled in&quot;);
		err++;
	}
	if ($(&quot;#roleId&quot;).val() == &quot;OFC&quot;){
		if ($(&quot;#nip&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field NIP should be filled in&quot;);
			err++;
		}
		if ($(&quot;#divId&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Staff should be filled in&quot;);
			err++;
		}
	}else if ($(&quot;#roleId&quot;).val() == &quot;MBR&quot;){
		if ($(&quot;#nim&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field NIM should be filled in&quot;);
			err++;
		}
		if ($(&quot;#majorsId&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Majors should be filled in&quot;);
			err++;
		}
		if ($(&quot;#prodiId&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Prodi should be filled in&quot;);
			err++;
		}
	}
	if(err == 0){
		return true;
	}else{
		return false;
	}
	
}
function fillComboRole(){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/getComboRole&quot;,
        type: &quot;get&quot;,
        data: {	},
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	// fill combo Prodi
        	var data = response.data;
        	$(&quot; , &quot;'&quot; , &quot;#roleId&quot; , &quot;'&quot; , &quot;).empty()
        	$(&quot; , &quot;'&quot; , &quot;#roleId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; selected=&quot;selected&quot;>-- Select One --&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	for(var i=0;  i&lt;data.length; i++){
        		$(&quot; , &quot;'&quot; , &quot;#roleId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; , &quot;'&quot; , &quot;+data[i].value+&quot; , &quot;'&quot; , &quot;&quot; >&quot; , &quot;'&quot; , &quot;+data[i].label+&quot; , &quot;'&quot; , &quot;&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	}
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}

function fillComboProdi(majorsIdData){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/getComboProdi&quot;,
        type: &quot;get&quot;,
        data: {majorsId:majorsIdData},
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	// fill combo Prodi
        	var data = response.data;
        	$(&quot; , &quot;'&quot; , &quot;#prodiId&quot; , &quot;'&quot; , &quot;).empty()
        	$(&quot; , &quot;'&quot; , &quot;#prodiId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; selected=&quot;selected&quot;>-- Select One--&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	for(var i=0;  i&lt;data.length; i++){
        		$(&quot; , &quot;'&quot; , &quot;#prodiId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; , &quot;'&quot; , &quot;+data[i].value+&quot; , &quot;'&quot; , &quot;&quot; >&quot; , &quot;'&quot; , &quot;+data[i].label+&quot; , &quot;'&quot; , &quot;&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	}
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}

function fillComboMajors(){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/getComboJurusan&quot;,
        type: &quot;get&quot;,
        data: {	},
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	// fill combo Jurusan
        	var data = response.data;
        	$(&quot; , &quot;'&quot; , &quot;#majorsId&quot; , &quot;'&quot; , &quot;).empty()
        	$(&quot; , &quot;'&quot; , &quot;#majorsId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; selected=&quot;selected&quot;>-- Select One--&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	for(var i=0;  i&lt;data.length; i++){
        		$(&quot; , &quot;'&quot; , &quot;#majorsId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; , &quot;'&quot; , &quot;+data[i].value+&quot; , &quot;'&quot; , &quot;&quot; >&quot; , &quot;'&quot; , &quot;+data[i].label+&quot; , &quot;'&quot; , &quot;&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	}
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}

function fillComboDivision(){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/getComboDivision&quot;,
        type: &quot;get&quot;,
        data: {	},
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	// fill combo Jurusan
        	var data = response.data;
        	$(&quot; , &quot;'&quot; , &quot;#divId&quot; , &quot;'&quot; , &quot;).empty()
        	$(&quot; , &quot;'&quot; , &quot;#divId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; selected=&quot;selected&quot;>-- Select One--&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	for(var i=0;  i&lt;data.length; i++){
        		$(&quot; , &quot;'&quot; , &quot;#divId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; , &quot;'&quot; , &quot;+data[i].value+&quot; , &quot;'&quot; , &quot;&quot; >&quot; , &quot;'&quot; , &quot;+data[i].label+&quot; , &quot;'&quot; , &quot;&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	}
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}
	


    
  
  

  
    
      Politeknik Negeri Bandung LIBRARY 2020
    
  
&quot;) or . = concat(&quot;

  

    

    
      
      



      POLBAN LIBRARY SYSTEM
    

    

    
      



      
      
        
          
          




            
              
              
                
                
                  
                
              
              Tampilkan Semua
            
          
          
          
          
            
       			USER PROFILE


            
            
              






              
              
              
              
                   PROFILE
              
              
                   LOG OUT 
              
            
          
        
      

    
  
  
  
    
    
      
      
        
          
          ADMINISTRATOR
        



      
      

      

      	
          
             Dashboard
          
        
      
        
          
             User Management
          
        
      
      
        
        
        
      
    
    
  

  
  
    
  	












	
		
			
				FORM EDIT USER
			
		
		
		
			
				
					
					Name *
					
					
						
							 
						
					
				
				
					
					Email *
					
					
						
							 
						
					
				
				
					
					Role *
					
					
						
							-- Select One --ADMINISTRATORMEMBEROFFICIAL
						
					
				
				
					
							
							NIP *
						
						
							
								
							
						
					
					
						
						Staff *
						
						
							
								-- Select One--Head Of LibraryLibrarian StaffOffice Boy
							
						
					
				
				
					
						
						NIM *
						
						
							
								
							
						
					
					
						
						Majors *
						
						
							
								-- Select One--Teknik ElektroTeknik MesinTeknik SipilTeknik Konversi EnergiTeknik Refrigerasi dan Tata UdaraTeknik Komputer dan InformatikaTeknik KimiaAkuntansiAdministrasi Niaga
							
						
					
					
						
						Prodi *
						
						
							
								-- Select One--DIII Teknik ListrikDIII Teknik ElektronikaDIII Teknik TelekomumnikasiDIV Teknik ElektronikaDIV Teknik TelekomunikasiDIV Teknik Otomasi IndustriDIII Teknik AeronautikaDIII Teknik MesinDIII Teknologi MesinDIV Proses ManufakturDIV Teknik Perancangan dan Konstruksi MesinDIII Konstruksi SipilDIII Konsentrasi GedungDIV Teknik Perancangan Jalan dan JembatanDIV Teknik Perawatan dan Perbaikan GedungDIII Teknik konversi EnergiDIV Teknik Konversi EnergiDIV Teknologi Pembangkit Tenaga ListrikDIII Teknik Refrigerasi dan Tata UdaraDIV Teknik Refrigerasi dan Tata UdaraDIII Teknik InformatikaDIV Teknik InformatikaDIII Teknik KimiaDIII Analis KimiaDIV Teknik Kimia Produksi BersihDIII AkuntansiDIII Keuangan PerbankanDIV AkuntansiDIV Akuntansi Manajemen PemerintahanDIV Keuangan SyariahDIII Administrasi BisnisDIII Manajemen PemasaranDIII Administrasi BisnisDIV Administrasi BisnisDIV Manajemen PemasaranDIV Manajemen Aset
							
						
					
				
				
					
					
					
						
							SUBMIT
						
						
							RESET
						
						
							CANCEL
						
					
					
				
			
		
	





$(document).ready(function() {
	fillComboRole();
	fillComboMajors();
	fillComboProdi(&quot;&quot;);
	fillComboDivision();
	setTimeout(function(){ initialData(); }, 1000);	
});
$(&quot;#majorsId&quot;).change(function(){
	fillComboProdi(this.value);
})

$(&quot;#roleId&quot;).change(function(){
	if(this.value == &quot;OFC&quot;){
		$(&quot;#formMember&quot;).hide();
		$(&quot;#formOfficial&quot;).show();
	}else if(this.value == &quot;MBR&quot;){
		$(&quot;#formMember&quot;).show();
		$(&quot;#formOfficial&quot;).hide();
	}else{
		$(&quot;#formMember&quot;).hide();
		$(&quot;#formOfficial&quot;).hide();
	}
	
})

$(&quot;#btnSubmit&quot;).click(function(){
	if(doValidation()){
		doSave();
	}
})

function initialData(){
	$(&quot;#name&quot;).val(&quot;Fauzan Lubos&quot;);
	$(&quot;#email&quot;).val(&quot;fauzan@fauh45.my.id&quot;);
	$(&quot;#roleId&quot;).val(&quot;OFC&quot;);
	$(&quot;#nip&quot;).val(&quot;6942069&quot;);
	$(&quot;#divId&quot;).val(&quot;S2&quot;);
	$(&quot;#nim&quot;).val(&quot;&quot;);
	$(&quot;#majorsId&quot;).val(&quot;&quot;);
	$(&quot;#prodiId&quot;).val(&quot;&quot;);
	if (&quot;EDIT&quot; == &quot;ADD&quot;){
		$(&quot;#formMember&quot;).hide();
		$(&quot;#formOfficial&quot;).hide();
	}else{
		if($(&quot;#roleId&quot;).val() == &quot;OFC&quot;){
			$(&quot;#formMember&quot;).hide();
			$(&quot;#formOfficial&quot;).show();
		}else if($(&quot;#roleId&quot;).val() == &quot;MBR&quot;){
			$(&quot;#formMember&quot;).show();
			$(&quot;#formOfficial&quot;).hide();
		}else{
			$(&quot;#formMember&quot;).hide();
			$(&quot;#formOfficial&quot;).hide();
		}
	}
}
function doSave(){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/save&quot;,
        type: &quot;post&quot;,
        data: {	name:$(&quot;#name&quot;).val(),
        		email:$(&quot;#email&quot;).val(),
        		role:$(&quot;#roleId&quot;).val(),
        		nim:$(&quot;#nim&quot;).val(),
        		majors:$(&quot;#majorsId&quot;).val(),
        		prodi:$(&quot;#prodiId&quot;).val(),
        		nip:$(&quot;#nip&quot;).val(),
        		division:$(&quot;#divId&quot;).val(),
        		mode: &quot;EDIT&quot; },
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	if (response.status){
        		setTimeout(function(){ window.location=&quot;/PerpustakaanWebApp/user&quot;; }, 1000);	
        	}
        	showMessage(response.code, response.message);
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}
$(&quot;#btnReset&quot;).click(function(){
	initialData()
// 	$(&quot;#name&quot;).val(&quot;&quot;);
// 	$(&quot;#email&quot;).val(&quot;&quot;);
// 	$(&quot;#roleId&quot;).val(&quot;&quot;);
// 	$(&quot;#nip&quot;).val(&quot;&quot;);
// 	$(&quot;#divId&quot;).val(&quot;&quot;);
// 	$(&quot;#nim&quot;).val(&quot;&quot;);
// 	$(&quot;#majorsId&quot;).val(&quot;&quot;);
// 	$(&quot;#prodiId&quot;).val(&quot;&quot;);
// 	$(&quot;#formMember&quot;).hide();
// 	$(&quot;#formOfficial&quot;).hide();
})
$(&quot;#btnCancel&quot;).click(function(){
	window.location=&quot;/PerpustakaanWebApp/user&quot;;	
})

function doValidation(){
	var err = 0;
	
	if ($(&quot;#name&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Name should be filled in&quot;);
		err++;
	}
	if ($(&quot;#email&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Email should be filled in&quot;);
		err++;
	}
	if ($(&quot;#roleId&quot;).val() == &quot;&quot;){
		showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Role should be filled in&quot;);
		err++;
	}
	if ($(&quot;#roleId&quot;).val() == &quot;OFC&quot;){
		if ($(&quot;#nip&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field NIP should be filled in&quot;);
			err++;
		}
		if ($(&quot;#divId&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Staff should be filled in&quot;);
			err++;
		}
	}else if ($(&quot;#roleId&quot;).val() == &quot;MBR&quot;){
		if ($(&quot;#nim&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field NIM should be filled in&quot;);
			err++;
		}
		if ($(&quot;#majorsId&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Majors should be filled in&quot;);
			err++;
		}
		if ($(&quot;#prodiId&quot;).val() == &quot;&quot;){
			showMessage(&quot;MSTD0031AERR&quot;,&quot;Field Prodi should be filled in&quot;);
			err++;
		}
	}
	if(err == 0){
		return true;
	}else{
		return false;
	}
	
}
function fillComboRole(){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/getComboRole&quot;,
        type: &quot;get&quot;,
        data: {	},
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	// fill combo Prodi
        	var data = response.data;
        	$(&quot; , &quot;'&quot; , &quot;#roleId&quot; , &quot;'&quot; , &quot;).empty()
        	$(&quot; , &quot;'&quot; , &quot;#roleId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; selected=&quot;selected&quot;>-- Select One --&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	for(var i=0;  i&lt;data.length; i++){
        		$(&quot; , &quot;'&quot; , &quot;#roleId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; , &quot;'&quot; , &quot;+data[i].value+&quot; , &quot;'&quot; , &quot;&quot; >&quot; , &quot;'&quot; , &quot;+data[i].label+&quot; , &quot;'&quot; , &quot;&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	}
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}

function fillComboProdi(majorsIdData){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/getComboProdi&quot;,
        type: &quot;get&quot;,
        data: {majorsId:majorsIdData},
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	// fill combo Prodi
        	var data = response.data;
        	$(&quot; , &quot;'&quot; , &quot;#prodiId&quot; , &quot;'&quot; , &quot;).empty()
        	$(&quot; , &quot;'&quot; , &quot;#prodiId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; selected=&quot;selected&quot;>-- Select One--&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	for(var i=0;  i&lt;data.length; i++){
        		$(&quot; , &quot;'&quot; , &quot;#prodiId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; , &quot;'&quot; , &quot;+data[i].value+&quot; , &quot;'&quot; , &quot;&quot; >&quot; , &quot;'&quot; , &quot;+data[i].label+&quot; , &quot;'&quot; , &quot;&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	}
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}

function fillComboMajors(){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/getComboJurusan&quot;,
        type: &quot;get&quot;,
        data: {	},
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	// fill combo Jurusan
        	var data = response.data;
        	$(&quot; , &quot;'&quot; , &quot;#majorsId&quot; , &quot;'&quot; , &quot;).empty()
        	$(&quot; , &quot;'&quot; , &quot;#majorsId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; selected=&quot;selected&quot;>-- Select One--&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	for(var i=0;  i&lt;data.length; i++){
        		$(&quot; , &quot;'&quot; , &quot;#majorsId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; , &quot;'&quot; , &quot;+data[i].value+&quot; , &quot;'&quot; , &quot;&quot; >&quot; , &quot;'&quot; , &quot;+data[i].label+&quot; , &quot;'&quot; , &quot;&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	}
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}

function fillComboDivision(){
	$.ajax({
        url: &quot;/PerpustakaanWebApp/api/user/getComboDivision&quot;,
        type: &quot;get&quot;,
        data: {	},
        contentType:&quot;application/x-www-form-urlencoded&quot;,
        dataType:&quot;json&quot;,
        success: function (response) {
        	// fill combo Jurusan
        	var data = response.data;
        	$(&quot; , &quot;'&quot; , &quot;#divId&quot; , &quot;'&quot; , &quot;).empty()
        	$(&quot; , &quot;'&quot; , &quot;#divId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; selected=&quot;selected&quot;>-- Select One--&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	for(var i=0;  i&lt;data.length; i++){
        		$(&quot; , &quot;'&quot; , &quot;#divId&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;option value=&quot;&quot; , &quot;'&quot; , &quot;+data[i].value+&quot; , &quot;'&quot; , &quot;&quot; >&quot; , &quot;'&quot; , &quot;+data[i].label+&quot; , &quot;'&quot; , &quot;&lt;/option>&quot; , &quot;'&quot; , &quot;);
        	}
        },
        error: function(jqXHR, textStatus, errorThrown) {
           console.log(textStatus, errorThrown);
        }
    });
}
	


    
  
  

  
    
      Politeknik Negeri Bandung LIBRARY 2020
    
  
&quot;))]</value>
   </webElementXpaths>
</WebElementEntity>
