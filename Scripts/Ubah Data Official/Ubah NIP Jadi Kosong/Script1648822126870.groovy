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

WebUI.waitForElementVisible(findTestObject('Object Repository/Perpustakaan Polban/Page_Sistem Perpustakaan POLBAN/input_User Name_username'), 
    0)

WebUI.verifyElementVisible(findTestObject('Object Repository/Perpustakaan Polban/Page_Sistem Perpustakaan POLBAN/input_Password_password'))

WebUI.verifyElementClickable(findTestObject('Object Repository/Perpustakaan Polban/Page_Sistem Perpustakaan POLBAN/button_LOGIN'))

WebUI.click(findTestObject('Object Repository/Perpustakaan Polban/Page_Sistem Perpustakaan POLBAN/input_User Name_username'))

WebUI.setText(findTestObject('Object Repository/Perpustakaan Polban/Page_Sistem Perpustakaan POLBAN/input_User Name_username'), 
    'januar@email.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Perpustakaan Polban/Page_Sistem Perpustakaan POLBAN/input_Password_password'), 
    'aeHFOx8jV/A=')

WebUI.click(findTestObject('Object Repository/Perpustakaan Polban/Page_Sistem Perpustakaan POLBAN/button_LOGIN'))

WebUI.verifyElementClickable(findTestObject('Object Repository/Perpustakaan Polban/Page_Perpustakaan/a_User Management'))

WebUI.click(findTestObject('Object Repository/Perpustakaan Polban/Page_Perpustakaan/span_User Management'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Perpustakaan Polban/Page_Perpustakaan/select_-- Select All --ADMINISTRATORMEMBEROFFICIAL'), 
    'OFC', true)

WebUI.setText(findTestObject('Object Repository/Perpustakaan Polban/Page_Perpustakaan/input_Name_name'), 'Fauzan Lubos')

WebUI.verifyElementClickable(findTestObject('Object Repository/Perpustakaan Polban/Page_Perpustakaan/button_Search'))

WebUI.click(findTestObject('Object Repository/Perpustakaan Polban/Page_Perpustakaan/button_Search'))

WebUI.verifyElementClickable(findTestObject('Object Repository/Perpustakaan Polban/Page_Perpustakaan/i_OFFICIAL_fa fa-pencil-square-o'))

WebUI.click(findTestObject('Object Repository/Perpustakaan Polban/Page_Perpustakaan/i_OFFICIAL_fa fa-pencil-square-o'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Perpustakaan Polban/Page_Perpustakaan/input__nip'), 0)

WebUI.click(findTestObject('Object Repository/Perpustakaan Polban/Page_Perpustakaan/div_POLBAN LIBRARY SYSTEM                  _1d888e'))

WebUI.setText(findTestObject('Object Repository/Perpustakaan Polban/Page_Perpustakaan/input__nip'), '')

WebUI.click(findTestObject('Object Repository/Perpustakaan Polban/Page_Perpustakaan/button_SUBMIT'))

WebUI.verifyElementText(findTestObject('Object Repository/Perpustakaan Polban/Page_Perpustakaan/div_Field NIP should be filled in'), 
    'Field NIP should be filled in')

WebUI.closeBrowser()

