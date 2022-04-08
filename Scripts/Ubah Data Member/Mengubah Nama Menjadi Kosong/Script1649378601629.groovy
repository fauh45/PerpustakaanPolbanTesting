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

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Sistem Perpustakaan POLBAN/input_User Name_username'), 
    0)

WebUI.setText(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Sistem Perpustakaan POLBAN/input_User Name_username'), 
    'januar@email.com')

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Sistem Perpustakaan POLBAN/input_Password_password'), 
    0)

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Sistem Perpustakaan POLBAN/input_Password_password'), 
    'aeHFOx8jV/A=')

WebUI.verifyElementClickable(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Sistem Perpustakaan POLBAN/button_LOGIN'))

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Sistem Perpustakaan POLBAN/button_LOGIN'))

WebUI.verifyElementClickable(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Perpustakaan/a_User Management'))

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Perpustakaan/span_User Management'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Perpustakaan/input_Name_name'), 
    0)

WebUI.setText(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Perpustakaan/input_Name_name'), 
    'Manis')

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Perpustakaan/button_Search'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Perpustakaan/i_MEMBER_fa fa-pencil-square-o'), 
    0)

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Perpustakaan/i_MEMBER_fa fa-pencil-square-o'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Perpustakaan/input_Name_name'), 
    0)

WebUI.setText(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Perpustakaan/input_Name_name'), 
    '')

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Perpustakaan/div_SUBMITRESETCANCEL'))

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Perpustakaan/button_SUBMIT'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Perpustakaan/div_Field Name should be filled in'), 
    0)

WebUI.verifyElementText(findTestObject('Object Repository/Page_Perpustakaan/Page_Perpustakaan/Page_Perpustakaan/div_Field Name should be filled in_1'), 
    'Field Name should be filled in')

WebUI.closeBrowser()

