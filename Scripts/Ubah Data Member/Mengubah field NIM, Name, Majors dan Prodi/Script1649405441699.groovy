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

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Perpustakaan/input_User Name_username'), 0)

WebUI.setText(findTestObject('Object Repository/Page_Perpustakaan/input_User Name_username'), 'januar@email.com')

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Perpustakaan/input_Password_password'), 0)

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Perpustakaan/input_Password_password'), 'aeHFOx8jV/A=')

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Perpustakaan/button_LOGIN'), 0)

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/button_LOGIN'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Perpustakaan/a_User Management'), 0)

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/a_User Management'))

WebUI.setText(findTestObject('Object Repository/Page_Perpustakaan/input_Name_name'), '')

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Perpustakaan/input_Name_name'), 0)

WebUI.setText(findTestObject('Object Repository/Page_Perpustakaan/input_Name_name'), 'Manis')

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Perpustakaan/button_Search'), 0)

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/button_Search'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Perpustakaan/i_MEMBER_fa fa-pencil-square-o'), 0)

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/i_MEMBER_fa fa-pencil-square-o'))

WebUI.doubleClick(findTestObject('Object Repository/Page_Perpustakaan/input_Name_name'))

WebUI.setText(findTestObject('Object Repository/Page_Perpustakaan/input_Name_name'), 'Manis')

WebUI.setText(findTestObject('Object Repository/Page_Perpustakaan/input__nim'), '191524000')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Perpustakaan/select_-- Select One--Teknik ElektroTeknik _583af6'), 
    'M06', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Perpustakaan/select_-- Select One--DIII Teknik Informati_c3b333'), 
    'P22', true)

WebUI.verifyElementClickable(findTestObject('Object Repository/Page_Perpustakaan/button_SUBMIT'))

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/button_SUBMIT'))

WebUI.waitForElementPresent(findTestObject('Page_Perpustakaan/div_users data successfully saved'), 0)

WebUI.verifyElementText(findTestObject('Page_Perpustakaan/div_users data successfully saved'), 'user\'s data successfully changed')

WebUI.closeBrowser()

