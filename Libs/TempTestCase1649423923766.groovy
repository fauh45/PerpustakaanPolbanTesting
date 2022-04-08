import com.kms.katalon.core.main.TestCaseMain
import com.kms.katalon.core.logging.KeywordLogger
import com.kms.katalon.core.testcase.TestCaseBinding
import com.kms.katalon.core.driver.internal.DriverCleanerCollector
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.webui.contribution.WebUiDriverCleaner
import com.kms.katalon.core.mobile.contribution.MobileDriverCleaner
import com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner
import com.kms.katalon.core.windows.keyword.contribution.WindowsDriverCleaner
import com.kms.katalon.core.testng.keyword.internal.TestNGDriverCleaner


DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.webui.contribution.WebUiDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.mobile.contribution.MobileDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.windows.keyword.contribution.WindowsDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.testng.keyword.internal.TestNGDriverCleaner())


RunConfiguration.setExecutionSettingFile('C:\\Users\\linda\\AppData\\Local\\Temp\\Katalon\\20220408_201843\\execution.properties')

TestCaseMain.beforeStart()

        TestCaseMain.runWSVerificationScript(new TestCaseBinding('',[:]), 'import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI\nimport com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile\nimport com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW\nimport com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS\nimport com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows\nimport static com.kms.katalon.core.testobject.ObjectRepository.findTestObject\nimport static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject\nimport static com.kms.katalon.core.testdata.TestDataFactory.findTestData\nimport static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase\nimport static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint\nimport com.kms.katalon.core.model.FailureHandling as FailureHandling\nimport com.kms.katalon.core.testcase.TestCase as TestCase\nimport com.kms.katalon.core.testdata.TestData as TestData\nimport com.kms.katalon.core.testobject.TestObject as TestObject\nimport com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint\nimport internal.GlobalVariable as GlobalVariable\nimport org.openqa.selenium.Keys as Keys\n\nWebUI.openBrowser(\'\')\n\nWebUI.navigateToUrl(\'http://103.31.39.208:8080/PerpustakaanWebApp/login\')\n\nWebUI.setText(findTestObject(\'Page_Sistem Perpustakaan POLBAN/input_User Name_username\'), \'januar@email.com\')\n\nWebUI.setEncryptedText(findTestObject(\'Page_Sistem Perpustakaan POLBAN/input_Password_password\'), \'aeHFOx8jV/A=\')\n\nWebUI.click(findTestObject(\'Page_Sistem Perpustakaan POLBAN/button_LOGIN\'))\n\nWebUI.click(findTestObject(\'Page_Perpustakaan/a_User Management\'))\n\nWebUI.click(findTestObject(\'Page_Perpustakaan/button_Add\'))\n\nWebUI.click(findTestObject(\'Page_Perpustakaan/input__name\'))\n\nWebUI.setText(findTestObject(\'Page_Perpustakaan/input__name\'), \'Iggy\')\n\nWebUI.setText(findTestObject(\'Page_Perpustakaan/input__email\'), \'ciodewanto2@gmail.coom\')\n\nWebUI.click(findTestObject(\'Page_Perpustakaan/input__email\'))\n\nWebUI.setText(findTestObject(\'Page_Perpustakaan/input__email\'), \'ciodewanto2@gmail.com\')\n\nWebUI.selectOptionByValue(findTestObject(\'Page_Perpustakaan/select_-- Select One --ADMINISTRATORMEMBEROFFICIAL\'), \'OFC\', \n    true)\n\nWebUI.setText(findTestObject(\'Page_Perpustakaan/input__nip\'), \'08123121\')\n\nWebUI.selectOptionByValue(findTestObject(\'Page_Perpustakaan/select_-- Select One--Head Of LibraryLibrar_69f11c\'), \'S2\', \n    true)\n\nWebUI.click(findTestObject(\'Page_Perpustakaan/button_SUBMIT\'))\n\nWebUI.click(findTestObject(\'Page_Perpustakaan/button_SUBMIT\'))\n\nWebUI.waitForElementPresent(findTestObject(\'Page_Perpustakaan/div_Cannot save data, because ciodewanto2gm_b93742\'), 0)\n\nWebUI.verifyElementPresent(findTestObject(\'Page_Perpustakaan/div_Cannot save data, because ciodewanto2gm_b93742_1\'), 0)\n\nWebUI.closeBrowser()\n\n', FailureHandling.STOP_ON_FAILURE, true)

