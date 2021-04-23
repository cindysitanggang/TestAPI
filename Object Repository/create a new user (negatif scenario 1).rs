<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>test API by entering first_name and last_name using a combination of letters, numbers, and special characters.</description>
   <name>create a new user (negatif scenario 1)</name>
   <tag></tag>
   <elementGuidId>4be075bb-ec4c-4293-9c13-db90878160c0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;    {\n      \&quot;email\&quot;:\&quot;michael.lawson@reqres.in\&quot;,\n      \&quot;first_name\&quot;:\&quot;Michael 12345\&quot;,\n      \&quot;last_name\&quot;:\&quot;Lawson !@#$%^\&quot;,\n      \&quot;avatar\&quot;:\&quot;https://reqres.in/img/faces/7-image.jpg\&quot;\n    }&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://reqres.in/api/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)

WS.verifyElementPropertyValue(response, 'email', 'michael.lawson@reqres.in')
WS.verifyElementPropertyValue(response, 'first_name', 'Michael 12345')
WS.verifyElementPropertyValue(response, 'last_name', 'Lawson !@#$%^')
WS.verifyElementPropertyValue(response, 'avatar', 'https://reqres.in/img/faces/7-image.jpg')

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
