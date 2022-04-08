import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://103.31.39.208:8080/PerpustakaanWebApp/login')

WebUI.setText(findTestObject('Object Repository/Page_Sistem Perpustakaan POLBAN/input_User Name_username'), 'januar@email.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Sistem Perpustakaan POLBAN/input_Password_password'), 'aeHFOx8jV/A=')

WebUI.verifyElementClickable(findTestObject('Object Repository/Page_Sistem Perpustakaan POLBAN/button_LOGIN'))

WebUI.click(findTestObject('Object Repository/Page_Sistem Perpustakaan POLBAN/button_LOGIN'))

WebUI.verifyElementClickable(findTestObject('Object Repository/Page_Perpustakaan/span_User Management'))

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/a_User Management'))

WebUI.setText(findTestObject('Object Repository/Page_Perpustakaan/input_Name_name'), 'Asri Maspupah')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Perpustakaan/select_-- Select All --ADMINISTRATORMEMBEROFFICIAL'), 
    'OFC', true)

WebUI.setText(findTestObject('Object Repository/Page_Perpustakaan/input_Description_description'), 'Head Of Library')

WebUI.verifyElementClickable(findTestObject('Object Repository/Page_Perpustakaan/button_Search'))

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/button_Search'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_Perpustakaan/div_MSTD0120AINF  Data berhasil ditampilkan'), 
    'MSTD0120AINF : Data berhasil ditampilkan.')

WebUI.closeBrowser()

