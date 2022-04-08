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

WebUI.click(findTestObject('Object Repository/Page_Sistem Perpustakaan POLBAN/button_LOGIN'))

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/a_User Management'))

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/button_Add'))

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/input__name'))

WebUI.setText(findTestObject('Object Repository/Page_Perpustakaan/input__name'), 'Iggy')

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/input__email'))

WebUI.setText(findTestObject('Object Repository/Page_Perpustakaan/input__email'), 'ciodewanto@gmail.com')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Perpustakaan/select_-- Select One --ADMINISTRATORMEMBEROFFICIAL'), 
    'OFC', true)

WebUI.setText(findTestObject('Object Repository/Page_Perpustakaan/input__nip'), '08123122')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Perpustakaan/select_-- Select One--Head Of LibraryLibrar_69f11c'), 
    'S2', true)

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/button_SUBMIT'))

WebUI.click(findTestObject('Object Repository/Page_Perpustakaan/button_SUBMIT'))

WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Perpustakaan/div_Cannot save data, because ciodewanto2gm_b93742'), 
    0)

WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Perpustakaan/div_Cannot save data, because ciodewanto2gm_b93742_1'), 
    0)

WebUI.closeBrowser()

